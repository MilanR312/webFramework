use std::{sync::Arc, collections::HashMap};

use futures_util::{stream::SplitSink, future::BoxFuture, Future};
use tokio::{net::TcpStream, sync::{Mutex, MutexGuard}};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use core::pin::Pin;
use async_trait::async_trait;



//creds https://stackoverflow.com/questions/64866474/vector-of-async-functions-that-receives-arguments-passed-by-reference
pub type Bf<'a> = Pin<Box<dyn Future<Output = ()> + 'a + Send>>;
pub type Handler<T> = Box<dyn for<'a> Fn(&'a mut T, &'a str) -> Bf<'a> + Sync + Send>;


#[async_trait]
pub trait Page{
    
    /// implemented automatically by the derive
    fn eval(&self, components: MutexGuard<'_, HashMap<String, Box<dyn Component + Sync + Send>>>) -> String;

    fn set_sender(&mut self, sender: Arc<Mutex<SplitSink<WebSocketStream<TcpStream>,Message>>>);
    
    /// implemented automatically by the derive
    /// 
    fn state_has_changed(&self);

    //setters for the fields
    //TODO: fix to allow for other types as argument, (maybe with Any?)
    async fn execute(&mut self, method: &str, arg: &str);

//    fn get_methods(&self) -> HashMap<&'static str, Handler<Self>>;
}
#[async_trait]
pub trait Component{
    ///implemented automatically
    fn eval(&self) -> String;

    async fn execute(&mut self, method: &str, arg: &str);
}


#[async_trait]
pub trait UserFunctions{
    async fn user_execute(&mut self, method: &str, arg: &str);

}


pub trait UserPage {
    fn new() -> Self;
}
use crate::html::HtmlRoot;
pub struct Html{
    pub content: HtmlRoot,
    pub id: String
}

pub struct AsyncFnPtr<T> {
    func: Box<dyn Fn(&mut T, &str) -> BoxFuture<'static, ()> + Send + Sync + 'static>
}

impl <T: 'static> AsyncFnPtr<T> {
    pub fn new<F>(f :fn(&mut T, &str) -> F) -> Self
    where F: Future<Output = ()> + Send + Sync + 'static {
        Self {
            func: Box::new(move |a,b| Box::pin(f(a,b)))
         }
    }

    pub async fn run(&self, page_self: &mut T, val: &str) {
        (self.func)(page_self, val).await;
    }
}

pub trait AsyncFn<G> {
    fn call(&self, s: &mut G, val: &str) -> BoxFuture<'static, ()>;
}
impl<T, F, G> AsyncFn<G> for T
where 
    T: Fn(&mut G, &str) -> F,
    F: Future<Output = ()> + 'static + Send,
{
    fn call(&self, s: &mut G, arg: &str) -> BoxFuture<'static, ()>{
        Box::pin(self(s, arg))
    }
}