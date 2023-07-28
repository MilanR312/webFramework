use std::{collections::HashMap, fmt::Display};

use tokio::io::{AsyncBufRead, AsyncBufReadExt, AsyncWrite, AsyncWriteExt};


#[derive(Debug, Hash)]
pub enum Method{
    Get,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub content: Option<String>
}
#[derive(Debug)]
pub enum Status {
    NotFound,
    ServerError,
    Ok
}
#[derive(Debug)]
pub struct Response {
    pub version: String,
    pub status: Status,
    pub headers: HashMap<String, String>,
    pub body: Option<String>
}
pub struct ResponseBuilder{
    build_target: Response
}


impl TryFrom<&str> for Method {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "GET" => Ok(Self::Get),
            method => Err(anyhow::anyhow!("unsupported method {method}"))
        }
    }
}

impl Request {
    pub async fn parse_request<T>(mut stream: T) -> anyhow::Result<Self> 
        where T: AsyncBufRead + Unpin
    {
        let mut line_buffer = String::new();
        stream.read_line(&mut line_buffer).await?;
        let mut parts = line_buffer.split_whitespace();

        let method: Method = parts.next()
            .ok_or(anyhow::anyhow!("missing method"))
            .and_then(TryInto::try_into)?;

        let path: String = parts.next()
            .ok_or(anyhow::anyhow!("missing path"))
            .map(Into::into)?;

        
        let mut headers = HashMap::new();
        loop {
            line_buffer.clear();
            stream.read_line(&mut line_buffer).await?;

            if line_buffer.is_empty() || line_buffer == "\n" || line_buffer == "\r\n" {
                break;
            }

            let mut header_line = line_buffer.split(":");
            let key = header_line.next()
                .ok_or(anyhow::anyhow!("missing header value"))?
                .trim();
            let value = header_line.next()
                .ok_or(anyhow::anyhow!("missing balue field"))?
                .trim();
            headers.insert(key.to_string(), value.to_string());

        }
        //let mut line_buffer = Vec::new();
        //stream.read_until(b'\n', &mut line_buffer).await?;
        //println!("remaining content = {}", line_buffer.len());
        Ok(Request { 
            method, 
            path, 
            headers, 
            content: None 
        })
    }

}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound => write!(f, "404 Not Found"),
            Self::ServerError => write!(f, "500 Internal server Error"),
            Self::Ok => write!(f, "200 Ok"),
        }
    }
}

impl Response {
    pub fn new(version: &str) -> Self{
        Self {
            version: format!("HTTP/{version}"),
            status: Status::ServerError,
            headers: HashMap::new(),
            body: None
        }
    }
    fn to_string(&self) -> String {
        let headers: String = self.headers
            .iter()
            .map(|(key, val)| format!("{key}: {val}\r\n"))
            .collect();

        let mut out = format!("{} {}\r\n{headers}\r\n", self.version, self.status);
        if headers.is_empty() {
            out += "\r\n";
        }
        if let Some(x) = &self.body {
            out += &x;
        }
        //info!(?out, "printing {}" , out.len());
        format!("{}", out)
    }
    pub async fn write<T>(self, stream: &mut T) -> anyhow::Result<()>
    where T: AsyncWrite + Unpin
    {
        stream.write_all(self.to_string().as_bytes())
        .await?;
        stream.flush().await?;
        Ok(())
    }
}


impl ResponseBuilder {
    pub fn from_version(version: &str) -> Self{
        Self{
            build_target: Response::new(version)
        }
    }
    pub fn set_status(mut self, status: Status) -> Self{
        self.build_target.status = status;
        self
    }
    pub fn add_to_headers(mut self, key: &str, val:&str) -> Self{
        self.build_target.headers.insert(key.to_owned(), val.to_owned());
        self
    }
    pub fn add_content(mut self, content: &str) -> Self {
        self.build_target.headers.insert("Content-Length".to_string(), format!("{}", content.len()));
        self.build_target.body = Some(content.to_owned());
        self
    }
    pub fn build(self) -> Response {
        self.build_target
    }
}

