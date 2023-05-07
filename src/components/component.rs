pub trait Component {
    fn sync(&self);
    fn get_attrs(&self) -> &Attributes;
}

#[derive(Debug)]
pub struct Attributes {
    pub path: String,
    pub deps_file: String,
    pub url: String,
}
