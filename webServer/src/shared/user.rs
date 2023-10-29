use macros::*;

html!(
    <p>hey |name| from component</p>
);

code!({
    #[updateAble]
    name: String;

    

    fn init(&mut self){
        self.name = "namer".to_string();
    }
});