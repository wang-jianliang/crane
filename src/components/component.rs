use pyo3::prelude::*;
use std::collections::HashMap;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::components::git_dependency::GitDependency;
use crate::components::solution::Solution;
use lazy_static::lazy_static;

use crate::errors::Error;

pub type ComponentID = usize;

#[derive(Debug)]
pub struct ComponentArena {
    components: HashMap<ComponentID, Component>,
    next_id: AtomicUsize,
}

impl ComponentArena {
    pub fn new() -> Self {
        ComponentArena {
            components: HashMap::new(),
            next_id: 0.into(),
        }
    }

    pub fn instance() -> &'static Mutex<Self> {
        lazy_static! {
            static ref INSTANCE: Mutex<ComponentArena> = Mutex::new(ComponentArena::new());
        }
        &INSTANCE
    }

    pub fn add(&mut self, component: Component) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        self.components.insert(id, component);
        id
    }

    pub fn get(&self, id: usize) -> Option<&Component> {
        self.components.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut Component> {
        self.components.get_mut(&id)
    }
}

#[derive(Debug)]
pub enum ComponentType {
    Solution,
    GitDependency,
}

pub trait FromPyObject {
    fn from_py(py_obj: &PyAny) -> Result<Self, PyErr>
    where
        Self: Sized;
}

#[derive(Debug)]
pub struct Component {
    pub name: String,
    pub type_: ComponentType,
    parent_id: Option<usize>,
    children_ids: Vec<usize>,
    impl_: Box<dyn ComponentImpl>,
}

impl Component {
    pub fn from_py(py_obj: &PyAny) -> Result<ComponentID, PyErr> {
        
        let name = py_obj.get_item("name")?.extract::<String>()?;
        let type_ = py_obj.get_item("type")?.extract::<String>()?;
        let type_ = match type_.as_str() {
            "solution" => ComponentType::Solution,
            "git_dependency" => ComponentType::GitDependency,
            _ => return Err(pyo3::exceptions::PyTypeError::new_err("unknown component type")),
        };
        let impl_: Box<dyn ComponentImpl> = match type_ {
            ComponentType::Solution => Box::new(Solution::from_py(py_obj)?),
            ComponentType::GitDependency => Box::new(GitDependency::from_py(py_obj)?),
        };

        let comp = Component {
            name,
            type_,
            parent_id: None,
            children_ids: Vec::new(),
            impl_,
        };
        let id = ComponentArena::instance().lock().unwrap().add(comp);

        Ok(id)
    }

    pub fn parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    pub fn set_parent_id(&mut self, parent_id: Option<usize>) {
        self.parent_id = parent_id;
    }

    pub fn children_ids(&self) -> Vec<usize> {
        self.children_ids.clone()
    }

    pub fn add_child(&mut self, child_id: ComponentID) {
        self.children_ids.push(child_id);
    }

    pub fn remove_child(&mut self, child_id: ComponentID) {
        self.children_ids.retain(|&id| id != child_id);
    }

    pub fn sync(&self) -> Result<(), Error> {
        self.impl_.sync(self)
    }
}

pub trait ComponentImpl: std::fmt::Debug + Send {
    fn sync(&self, comp: &Component) -> Result<(), Error>;
}

// #[async_trait]
// pub trait Component: std::fmt::Debug + Send {
//     async fn sync(&self) -> Result<(), Error>;
//     fn name(&self) -> String;
//     fn parent(&self) -> Option<Box<dyn Component>>;
//     fn set_parent(&mut self, parent: Option<Box<dyn Component>>);
//     fn children(&self) -> Vec<Box<dyn Component>>;
//     fn add_child(&mut self, child: Box<dyn Component>);
// }
