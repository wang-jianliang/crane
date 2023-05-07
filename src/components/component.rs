pub trait Component {
    fn new(name: &String, path: &String) -> Self;
    fn sync(&self);
    fn info(&self) -> &ComponentInfo;
}

#[derive(Debug)]
pub struct ComponentInfo {
    pub path: String,
    pub name: String,
}
