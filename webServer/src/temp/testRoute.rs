
use std::{sync::Arc, pin::Pin};

use achernar::page::*;
use macros::*;
use futures_util::{SinkExt, StreamExt, future, stream::SplitSink, Future};
use future::BoxFuture;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::{WebSocket, Message}, WebSocketStream};
use std::collections::HashMap;
use tracing::info;

route!("/testRoute");

html!(
    <script> 
        "let a = 5;"
        "function incr(){ a++; conn.send('/testRoute,set_test,' + String(a))}"
    </script>
    <p>"this is a testing rust page |test|"</p>
    "<button onclick='incr();'>click here</button>"
    //todo implement this to allow functions (encode functions into the hashmap)
    //<button onclick=|incr|> click here </button>
    //(() => conn.send('/testRoute, incr'))()
);

#[derive(Page)]
pub struct TestRoute{
    #[system]
    sender: Option<Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>>,
    #[updateAble]
    test: u32
}
//add the proc macro executable that implements all 
//remaining functions that need to get put into the hashmap
//#[executable]
impl TestRoute{
    fn incr(&mut self){
        self.set_test(&format!("{}",self.test + 1));
    }
}

impl UserPage for TestRoute{
    fn new() -> Self {
        
        Self {  
            sender: None,
            test: 5
        }
    }
}

