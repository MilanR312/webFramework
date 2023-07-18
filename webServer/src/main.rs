#![feature(trace_macros)]

trace_macros!(false);

use std::{
    net::{
        TcpListener, 
        TcpStream, SocketAddr
    }, 
    println, 
    io::{
        BufReader,
        prelude::*, BufWriter
    },
    fs, collections::{HashMap}, path::PathBuf, hash::{Hash}, thread::{Thread, JoinHandle}, time
};

use pages::{Page, new_from_string};
use sha1::{Sha1, Digest};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
mod pages;

mod http_return;
use crate::http_return::{HttpReturn, HttpReturnBuilder};

struct WebSocket{
    conn: TcpStream,
}

impl WebSocket {
    fn new(conn: TcpStream) -> Self {
        Self {
            conn
        }
    }
    fn send(&mut self, data: &str){
        let mut packet: Vec<u8> = Vec::new();
        packet.push(0b10000001);
        let len = data.len();
        packet.push(len as u8);
        for item in data.bytes(){
            packet.push(item);
        }   
        let r = self.conn.write_all(packet.as_slice());
        println!("write result = {:?}", r);
        let r = self.conn.flush();
        println!("flush result = {:?}", r);
    }
}


struct HttpConnection{
    loaded_pages: HashMap<String, Box<dyn Page>>,
    thr : Option<JoinHandle<()>>, //none if no websocket is open,
    tx: Option<Sender<String>>
}

impl HttpConnection{

    fn from_stream() -> Self{
        Self {
            loaded_pages: HashMap::new(),
            thr: None,
            tx: None
        }
    }
    fn make_channels(&mut self) -> Receiver<String>{
        let (tx, rx): (Sender<String>, Receiver<String>) = mpsc::channel();
        self.tx = Some(tx.clone());
        for (page_name, page) in self.loaded_pages.iter_mut(){
            println!("setting tx for {}", page_name);
            page.set_tx(tx.clone())
        }
        rx
    }
    fn handle_connection(&mut self, mut stream: TcpStream){
        let buf_reader = BufReader::new(&mut stream);
    
        let mut http_request = buf_reader
            .lines();
    
        let header = http_request.next();
        println!("header = {:?}", header);
        let header = match header {
            Some(Ok(h)) => h,
            _ =>  return self.send(
                &mut stream,
                HttpReturnBuilder::internal_error()
            )
        };
        //hashmap of all headers
        let headers: HashMap<_,_> = http_request.
            map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (a,b) = line.split_once(": ").unwrap();
                return (a.to_owned(), b.to_owned());
            })
            .collect();
        if let Some( key) = headers.get("Sec-WebSocket-Key") {
            //we should upgrade to websocket
            //first calculate the hash of the key
            let mut key = key.to_owned();
            let mut hasher = Sha1::new();

            let websocket_uuid = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";
            key.push_str(websocket_uuid);


            hasher.update(key);
            let hashed = hasher.finalize();
            let hashed: [u8; 20] = hashed.as_slice().try_into().expect("wrong size");
            println!("{:?}", hashed);
            let hashed = base64::encode(hashed);

            println!("upgrading to websocket");
            self.send(&mut stream, HttpReturnBuilder::from_version("1.1")
            .set_code(101, "Switching protocols")
            .add_to_headers("Upgrade: websocket")
            .add_to_headers("Connection: Upgrade")
            .add_to_headers(&format!("Sec-WebSocket-Accept: {}", hashed))
            .build()
            );

            let mut connection = WebSocket::new(
                stream.try_clone().expect("error cloning stream"));
            let tx = self.make_channels();
            self.thr = Some(thread::spawn(move || {
                let tx = tx;
                thread::sleep(time::Duration::from_secs(5));
                println!("-------starting thread------");
                loop {
                    let data = tx.recv();
                    println!("sending = {:?}", data);
                    if let Ok(x) = data {
                        connection.send(&x);
                    }
                }
            }));
            return;
        }
        println!("header data = {header}");
        let mut header_content = header.split(" ").into_iter();
    
    
        let req = header_content.next();
        let path = header_content.next().map(|val|  "./webServer/src/pages".to_string() + val);
        
        match req {
            Some("GET") => {
                let content = self.get(path.as_deref());
                self.send(&mut stream, content);
            },
            Some("POST") => {
                let path = path.unwrap();
                let (path, instruction) = path.split_at(path.rfind("/").unwrap());
                self.post(&mut stream, Some(path))
            },
            _ => self.send(
                &mut stream,
                HttpReturnBuilder::not_implemented()
            )
        }
    }

    fn send(&self, stream: &mut TcpStream, content: HttpReturn){
        stream.write_all(content.to_string().as_bytes()).unwrap();
    }
    fn get(&mut self,path: Option<&str>) -> HttpReturn{
        println!("path = {:?}", path);
        let path = match path {
            None => return HttpReturnBuilder::from_version("1.1")
                        .set_code(301, "moved")
                        .add_to_headers("Location: /Default.html")
                        .build()
                    ,
            Some(p) => p
        };
            //regular page so just load it in
        if !path.ends_with(".rs") {
            let file = fs::read_to_string(path);
            println!("file = {:?}", file);
            let file = match file {
                Err(_) => return HttpReturnBuilder::from_version("1.1")
                    .set_code(301, "moved")
                    .add_to_headers("Location: /Default.html")
                    .build(),
                Ok(p) => p,
            };
            
            return HttpReturnBuilder::from_version("1.1")
                    .set_code(200, "OK")
                    .add_content(file.trim())
                    .build()
            

        } else {
            let x: PathBuf = path.into();
            let file_name = x.file_stem().map(|v| v.to_str().expect("err abc")).expect("err abe");
            
            //insert if not exists
            if !self.loaded_pages.contains_key(path){
                let added_page = new_from_string(file_name);
                self.loaded_pages.insert(path.to_owned(), added_page);
            }
            let page = self.loaded_pages.get(path).unwrap();
            println!("added page at {}", path);
            //fill in the page
            let content = page.eval();
            println!("amount of loaded pages in get = {}", self.loaded_pages.len());
            return 
                HttpReturnBuilder::from_version("1.1")
                    .set_code(200, "ok")
                    .add_content(&content)
                    //.add_to_headers("Connection: Keep-Alive")
                    .build()
            
        }
    }

    fn post(&self, stream:&mut TcpStream, path: Option<&str>){
        println!("POST ---------------------");
        let path = match path {
            None => return self.send(stream,
                HttpReturnBuilder::from_version("1.1")
                        .set_code(301, "moved")
                        .add_to_headers("Location: /Default.html")
                        .build()
                    ),
            Some(p) => p
        };
        println!("PATH = {}", path);
        println!("amount of loaded pages in post = {}", self.loaded_pages.len());

        println!("loaded pages =");
        for (key, val) in self.loaded_pages.iter(){
            println!("key = {key}");
        }
        let page = self.loaded_pages.get(path);
        let page = match page {
            None => return self.send(
                stream,
                HttpReturnBuilder::from_version("1.1")
                        .set_code(301, "moved")
                        .add_to_headers("Location: /Default.html")
                        .build()
                    ),
            Some(p) => p,
        };
        println!("page loaded");
        println!("content = {}", page.eval());

        return self.send(
            stream,
            HttpReturnBuilder::from_version("1.1")
            .set_code(200, "ok")
            .build());
    }

}



fn main() {

    let listener = TcpListener::bind("127.0.0.1:5098").expect("binding failed");
    let mut connections: HashMap<SocketAddr,HttpConnection> = HashMap::new();
    for connection in listener.incoming(){
        let connection = connection.unwrap();
        println!("address = {:?}", connection.local_addr());
        let address = match connection.local_addr() {
            Ok(a) => a,
            _ => panic!("invalid socket")
        };

        //let mut nw = HttpConnection::from_stream(connection);
        //nw.answer();
        println!("starting transaction");
        if !connections.contains_key(&address){
            println!("added new connection");
            
            //connections.insert(address, nw);
            connections.insert(address, HttpConnection::from_stream());
        }
        let conn: &mut HttpConnection = connections.get_mut(&address).unwrap();
        conn.handle_connection(connection);
        println!("finished loop");
    }
}
