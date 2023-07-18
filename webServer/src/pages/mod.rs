pub mod clicker;


use std::sync::mpsc::Sender;

use serde_json::Value;
pub trait Page{
    fn set_tx(&mut self, val: Sender<String>);
    fn to_json(&self) -> Value;
    fn eval(&self) -> String;
    fn state_has_changed(&self);
}

pub fn new_from_string(name: &str) -> Box<dyn Page>{
    println!("path = {}", name);
    match name {
        "clicker" => Box::new(clicker::Clicker::new()),
        _ => panic!("unknown page")
    }
}