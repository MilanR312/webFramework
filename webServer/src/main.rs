#![feature(fn_traits)]

mod shared;
mod pages;
mod http_return;

//TODO:
//fix functions by using dynamic scheduler or saving the types with the function name and checking first
//make components

use achernar;
use macros::load_components;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use macros::load_pages;
    use achernar::page::*;

    let set = achernar::settings::Settings {
        ip_address: "127.0.0.1".to_string(),
        port: 8081,
        shared_folder: load_components!("./webServer/src/shared"),
        pages_folder: load_pages!("./webServer/src/pages")
    };

    achernar::init(set).await
    /*let listener = TcpListener::bind("127.0.0.1:5098").expect("binding failed");
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
    }*/
}
