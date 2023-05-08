use crate::components::component::{Component, ComponentInfo};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Solution {
    info: ComponentInfo,
    deps_file: Option<String>,
}

impl Component for Solution {
    fn new(
        name: &String,
        path: &String,
        source_stamp: &String,
        extra_attrs: HashMap<String, String>,
    ) -> Self {
        let deps_file = match extra_attrs.get(&String::from("deps_file")) {
            Some(attr) => Some(attr.clone()),
            None => None,
        };
        Solution {
            info: ComponentInfo::new(name, path, source_stamp),
            deps_file: deps_file,
        }
    }

    fn sync(&self) {
        println!("sync");
    }

    fn info(&self) -> &ComponentInfo {
        &self.info
    }
}
