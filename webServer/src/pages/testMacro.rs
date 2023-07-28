use macros::*;


route!("/testRoute");

html!(
    <p>this is a testing rust page |test|</p>
    <button onclick=|incr|>
        incr
    </button>
    <button onclick=|decr|>
        decr
    </button>
);

code!(
    {
        #[updateAble]
        test: u8;
        #[updateAble]
        clk: u8;
    
        fn init(&mut self){
            self.test = 8;
            self.clk = 2;
        }

        async fn incr(&mut self, _val: &str){
            self.set_test(&(self.test + 1).to_string()).await;
            self.set_clk(&(self.clk + 1).to_string()).await;
        }
        async fn decr(&mut self, _val: &str){
            self.set_test(&(self.test - 1).to_string()).await;
            self.set_clk(&(self.clk - 1).to_string()).await;
        }
    }
);
