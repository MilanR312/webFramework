use macros::Page;
pub trait Page{
    fn to_json(&self);
}

#[derive(Page)]
pub struct test{
    count: u8
}

impl test {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }
}