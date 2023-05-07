use crate::components::component::{Attributes, Component};

#[derive(Debug)]
pub struct Solution {
    pub comp_attrs: Attributes,
}

impl Component for Solution {
    fn sync(&self) {
        println!("sync");
    }

    fn get_attrs(&self) -> &Attributes {
        &self.comp_attrs
    }
}
