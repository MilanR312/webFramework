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
    <button onclick=|reset|>
        reset
    </button>
    <user />
);

code!(
    {
        #[updateAble]
        test: i8;

        fn init(&mut self){
            self.test = 8;
        }

        async fn incr(&mut self, _val: i8){
            self.set_test(self.test + 1).await;
        }
        async fn decr(&mut self, _val: i8){
            self.set_test(self.test - 1).await;
        }
        async fn reset(&mut self, val: i8){
            self.set_test(8).await;
        }
    }
);
