use crate::components::component::{Component, ComponentInfo};

#[derive(Debug)]
pub struct Solution {
    pub info: ComponentInfo,
}

impl Component for Solution {
    fn new(name: &String, path: &String) -> Self {
        Solution {
            info: ComponentInfo {
                name: name.to_string(),
                path: path.to_string(),
            },
        }
    }

    fn sync(&self) {
        println!("sync");
    }

    fn info(&self) -> &ComponentInfo {
        &self.info
    }
}
