
#[derive(Clone)]
pub struct HttpReturn{
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<String>,
    body: Option<String>
}

impl HttpReturn {
    
    pub fn new(version: &str) -> Self {
        Self {
            version: format!("HTTP/{version}"),
            status_code: 500,
            reason: "Internal server Error".to_string(),
            headers: Vec::new(),
            body: None
        }
    }
    pub fn to_string(self) -> String {
        let headers: String = self.headers.into_iter()
            .map(|val| format!("{val}\r\n")).collect();

        let mut out = format!("{} {} {}\r\n{}\r\n", self.version, self.status_code, self.reason, headers);
        if headers.is_empty() {
            out += "\r\n";
        }
        if let Some(x) = self.body {
            out += &x;
        }
        println!("test = {}", out);
        out
    }
}



pub struct HttpReturnBuilder{
    build_target: HttpReturn,
}

impl HttpReturnBuilder{
    pub fn from_version(version: &str) -> Self{
        HttpReturnBuilder {
            build_target: HttpReturn::new(version)
        }
    }
    pub fn set_code(mut self, code: u32, reason: &str) -> Self{
        self.build_target.status_code = code;
        self.build_target.reason = reason.to_owned();
        self
    }
    pub fn add_to_headers(mut self, header: &str) -> Self{
        self.build_target.headers.push(header.to_string());
        self
    }
    pub fn add_content(mut self, content: &str) -> Self{
        self.build_target.headers.push(format!("Content-Length: {}", content.len()));
        self.build_target.body = Some(content.to_owned());
        self
    }   
    pub fn build(self) -> HttpReturn {
        self.build_target.clone()
    }
    pub fn internal_error() -> HttpReturn {
        Self::from_version("1.1")
        .set_code(500, "Internal Server Error")
        .build()
    }
    pub fn not_implemented() -> HttpReturn {
        Self::from_version("1.1")
        .set_code(501, "not implemented")
        .build()
    }
    pub fn moved(file: &str) -> HttpReturn {
        Self::from_version("1.1")
        .set_code(301, "moved")
        .add_to_headers(&format!("Location: /{}", file))
        .build()
    }
}