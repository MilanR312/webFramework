use std::{fs, sync::Arc};

use https::Request;
use settings::Settings;

use tokio::{io::BufStream, net::{TcpListener, TcpStream}, sync::Mutex};
use tokio_tungstenite::accept_async;
use tracing::info;
use futures_util::{SinkExt, StreamExt};
use crate::https::ResponseBuilder;
mod https;
pub mod settings;

//use macros::*;
pub mod page;

pub async fn init(settings: Settings) -> anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    
    let listener = TcpListener::bind(
        format!("{}:{}", settings.ip_address, settings.port)
    ).await.unwrap();

    let web_asm_listener = TcpListener::bind(
        format!("{}:{}", settings.ip_address, settings.port + 1)
    ).await.unwrap();

    info!("listening on: {}", listener.local_addr()?);
    let default_page = fs::read_to_string("./webServer/src/pages/default.html").expect("default file not found");
    let set2 = settings.clone();
    //http loop
    tokio::spawn(async move {
        loop {
            let (stream, addr) = match listener.accept().await{
                Ok(v) => v,
                Err(_) => continue,
            };
            let mut stream = BufStream::new(stream);
            let default_page = default_page.clone();
            let set = settings.clone();
            tokio::spawn(async move {
                info!(?addr, "new connection");



                if let Ok(request) = Request::parse_request(&mut stream).await {

                    let to_send = get_page(&request.path, &set).await;

                    let _ = send(&mut stream, default_page, to_send.as_deref()).await;
                }

            });
        }
    });

    //webSocket loop
    loop {
        let (stream, _addr) = web_asm_listener.accept().await.expect("failed to accept");
        info!("new websocket connection");
        let settings: Settings = set2.clone();

        tokio::spawn(async move {
            let ws = accept_async(stream).await.expect("error accepting");
            let (writer
                , mut reader) = ws.split();
            
            let tx = Arc::new(Mutex::new(writer));
            {
                settings.pages_folder.lock().await
                    .iter_mut()
                    .for_each(|(_key, val)|{
                        val.set_sender(tx.clone());
                    })
            }
        
            let _ = tx.lock().await.flush().await;
            loop {
                if let Some(Ok(msg)) = reader.next().await {
                    info!(?msg, "got message");
                    if msg.is_close(){
                        break;
                    }

                    if msg.is_text() {
                        //try to get the function
                        {
                            let msg = msg.to_string();
                            let mut msg = msg.split(",");
                            let current_page = msg.next();
                            let func = msg.next();
                            let arg = msg.next();
                            let (path, func, arg) = match (current_page, func, arg) {
                                (Some(a), Some(b), Some(c)) => (a,b, c),
                                _ => {
                                    let mut s = tx.lock().await;
                                    let _ = s.send(tokio_tungstenite::tungstenite::Message::Text(
                                        "invalid args".to_string()
                                    )).await;
                                    let _ = s.flush().await;
                                    continue;
                                }
                            };
                            
                            let mut page = settings.pages_folder.lock()
                                .await;
                            let page = page.get_mut(path);
                            if let Some(page) = page {
                                info!("found page");
                                page.execute(func, arg).await;
                                info!("executed method");
                            }
                        }
                    }
                }
            }
        });

    }

}
async fn send(stream: &mut BufStream<TcpStream>, default_page: String, to_send: Option<&str>) -> Result<(), ()>{
    let respons = ResponseBuilder::from_version("1.1");
    let (respons, content) = match to_send {
        Some(content) => (respons.set_status(https::Status::Ok), content),
        None => (respons.set_status(https::Status::NotFound), "404 file not found")
    };
    //info!(?content, "filling in");
    let to_send = default_page.replace("|body|", content);
    //info!(?to_send, "filling in");

    respons.add_content(&to_send).build()
        .write(stream).await.unwrap();
    Ok(())
}
async fn get_page(path: &str, settings: &Settings) -> Option<String> {
    //check if the requested path exists in the rust pages
    let binding = settings.pages_folder
        .lock().await;
    
    //if it is return the string equivalent
    if let Some(val) = binding.get(path) {
        let content = val.eval();
        info!(?content, "found rust page");

        return Some(content);
    }

    //check if it exists as a html file
    let mut full_path = path.to_owned();
    full_path.push_str(".html");
    let file = fs::read_to_string(
        format!("./webServer/src/pages{}", full_path)
    );
    if let Ok(val) = file {
        info!("found html page");
        return Some(val);
    }
    info!("found no page");
    None
}