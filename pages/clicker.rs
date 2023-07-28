use std::{fs, sync::mpsc::Sender, thread};
use handlebars::Handlebars;
use serde_json::{json, Value};

use super::Page;
use macros::Page;
use macros::route;
//use macros::to_json;

route!("/");


static PAGE : &str= " \
    <div> \
    <p>hello {name} </p>
    {clk}
    <p>you clicked {clk} times</p> \
    <form method=\"POST\" action=\"/clicker.rs/incr\"> \
        <!-- ... --> \
        <input type=\"submit\"> \
    </form> \
    </div>  \
";
#[derive(Page)]
pub struct Clicker{
    #[system]
    sender: Option<Sender<String>>,
    clk: u8,
    #[updateAble]
    incr: u8,
    #[updateAble]
    name: String
}

impl Clicker {
    pub fn new() -> Self {
        Self {
            sender: None,
            clk: 0,
            incr: 5,
            name: "test".to_string()
        }
    }
    
    //default
    
}