use crate::components::component::{Component, ComponentInfo};

#[derive(Debug)]
pub struct Solution {
    pub info: ComponentInfo,
}

impl Component for Solution {
    fn new(name: &String, path: &String, source_stamp: &String) -> Self {
        Solution {
            info: ComponentInfo::new( 
                name,
                path,
                source_stamp,
            ),
        }
    }

    fn sync(&self) {
        println!("sync");
    }

    fn info(&self) -> &ComponentInfo {
        &self.info
    }
}
