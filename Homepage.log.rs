use std :: sync :: Arc ; use achernar :: page :: * ; use futures_util ::
{ SinkExt, stream :: SplitSink } ; use tokio ::
{ net :: TcpStream, sync :: Mutex } ; use tokio_tungstenite ::
{ tungstenite :: Message, WebSocketStream } ; use std :: collections ::
HashMap ; use tracing :: info ; #[derive(Page)] pub struct Homepage
{
    #[system] sender : Option < Arc < Mutex < SplitSink < WebSocketStream <
    TcpStream >, Message >> >>, name : String,
} impl Homepage { fn init(& mut self) { self.name = "user".to_string() ; } }
use achernar :: page :: * ; #[async_trait] impl UserFunctions for Homepage
{
    async fn user_execute(& mut self, method : & str, arg : & str)
    {
        info! ("searching for user methods") ; let methods =
        {
            let mut out : HashMap < & 'static str, Handler < Self >> = HashMap
            :: new() ; out
        } ; if let Some(func) = methods.get(method)
        {
            info! ("executing user method") ; (func) (self, arg).await ;
            return ;
        }
    }
} impl Homepage
{
    pub fn new() -> Self
    {
        let mut out = Self { sender : None, name : < String > :: default(), }
        ; Self :: init(& mut out) ; out
    }
}