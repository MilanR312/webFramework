use tokio::sync::Mutex;

use crate::page::{Page, Component};
use std::{collections::HashMap, sync::Arc};




#[derive(Clone)]
pub struct Settings{
    pub shared_folder: Arc<Mutex<HashMap<String, Box<dyn Component + Sync + Send>>>>,
    pub pages_folder:  Arc<Mutex<HashMap<String, Box<dyn Page + Sync + Send>>>>,
    pub ip_address: String,
    pub port: u16,   
}


