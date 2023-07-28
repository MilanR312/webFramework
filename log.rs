use std :: sync :: Arc ; use achernar :: page :: * ; use futures_util ::
{ SinkExt, stream :: SplitSink } ; use tokio ::
{ net :: TcpStream, sync :: Mutex } ; use tokio_tungstenite ::
{ tungstenite :: Message, WebSocketStream } ; use std :: collections ::
HashMap ; use tracing :: info ; #[derive(Page)] pub struct TestMacro
{
    #[system] sender : Option < Arc < Mutex < SplitSink < WebSocketStream <
    TcpStream >, Message >> >>, #[updateAble] test : u8, #[updateAble] clk :
    u8,
} impl TestMacro
{
    async fn incr(& mut self, _val : & str)
    {
        self.set_test(& (self.test + 1).to_string()).await ;
        self.set_clk(& (self.clk + 1).to_string()).await ;
    } async fn decr(& mut self, _val : & str)
    {
        self.set_test(& (self.test - 1).to_string()).await ;
        self.set_clk(& (self.clk - 1).to_string()).await ;
    }
} use achernar :: page :: * ; #[async_trait] impl UserFunctions for TestMacro
{
    async fn user_execute(& mut self, method : & str, arg : & str)
    {
        info! ("searching for user methods") ; let methods =
        {
            let mut out : HashMap < & 'static str, Handler < Self >> = HashMap
            :: new() ;
            out.insert("incr", Box ::
            new(| a, b | Box :: pin(Self :: incr(a, b)))) ;
            out.insert("decr", Box ::
            new(| a, b | Box :: pin(Self :: decr(a, b)))) ; out
        } ; if let Some(func) = methods.get(method)
        {
            info! ("executing user method") ; (func) (self, arg).await ;
            return ;
        }
    }
}