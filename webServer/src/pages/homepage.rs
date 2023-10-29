use macros::*;

route!("/");
html_from_file!("./webServer/src/pages/homepage.html");

code!(
    {
        name: String;

        fn init(&mut self){
            self.name = "user".to_string();
        }

    }
);
