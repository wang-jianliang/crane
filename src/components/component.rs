use std::any::Any;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;

use crate::components::git_dependency::GitDependency;
use crate::components::solution::Solution;
use crate::errors::Error;
use crate::visitors::{
    component_sync_visitor::ComponentSyncVisitor, component_visitor::ComponentVisitor,
};
use futures::future::try_join_all;
use lazy_static::lazy_static;
use parking_lot::{MappedMutexGuard, Mutex, MutexGuard};
use pyo3::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

pub type ComponentID = usize;

#[derive(Debug)]
pub struct ComponentArena {
    components: Mutex<Vec<Component>>,
}

impl ComponentArena {
    pub fn new() -> Self {
        ComponentArena {
            components: Mutex::new(Vec::<Component>::new()),
        }
    }

    pub fn instance() -> &'static Self {
        lazy_static! {
            static ref INSTANCE: ComponentArena = ComponentArena::new();
        }
        &INSTANCE
    }

    pub fn add(&self, component: Component) -> usize {
        let mut lock = self
            .components
            .try_lock_for(Duration::from_secs(10))
            .expect("Failed to lock components");
        let id = lock.len();
        lock.push(component);
        id
    }

    pub fn get(&self, id: usize) -> Option<MappedMutexGuard<Component>> {
        let lock = self
            .components
            .try_lock_for(Duration::from_secs(10))
            .expect("Failed to lock components");
        if id < lock.len() {
            Some(MutexGuard::map(lock, |components| &mut components[id]))
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub enum ComponentType {
    Unkonwn,
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
    children: Vec<usize>,
    pub impl_: Box<dyn ComponentImpl>,
}

impl Component {
    pub fn from_py(py_obj: &PyAny) -> Result<ComponentID, PyErr> {
        let name = py_obj.get_item("name")?.extract::<String>()?;
        let type_str = py_obj.get_item("type")?.extract::<String>()?;
        let type_ = match type_str.as_str() {
            "solution" => ComponentType::Solution,
            "git" => ComponentType::GitDependency,
            _ => ComponentType::Unkonwn,
        };
        let impl_: Box<dyn ComponentImpl> = match type_ {
            ComponentType::Solution => Box::new(Solution::from_py(py_obj)?),
            ComponentType::GitDependency => Box::new(GitDependency::from_py(py_obj)?),
            _ => {
                return Err(pyo3::exceptions::PyTypeError::new_err(
                    "unknown component type: ".to_owned() + &type_str,
                ))
            }
        };

        let comp = Component {
            name,
            type_,
            parent_id: None,
            children: Vec::new(),
            impl_,
        };
        println!("before lock");
        let id = ComponentArena::instance().add(comp);

        Ok(id)
    }

    pub fn set_parent_id(&mut self, parent_id: Option<usize>) {
        self.parent_id = parent_id;
    }

    pub fn add_child(&mut self, child_id: ComponentID) {
        self.children.push(child_id);
    }

    pub fn remove_child(&mut self, child_id: ComponentID) {
        self.children.retain(|&id| id != child_id);
    }
}

pub trait ComponentImpl: std::fmt::Debug + Send {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub fn visit_component<V: ComponentVisitor>(id: ComponentID, visitor: V) -> Result<(), Error> {
    let type_;
    {
        if let Some(comp) = ComponentArena::instance().get(id) {
            type_ = comp.type_.clone();
        } else {
            return Err(Error {
                message: String::from("unknown component id"),
            });
        }
    }
    match type_ {
        ComponentType::Solution => visitor.visit_solution(id),
        ComponentType::GitDependency => visitor.visit_git(id),
        _ => Err(Error {
            message: String::from("unknown component type"),
        }),
    }
}

pub async fn walk_components<V>(component_ids: Vec<ComponentID>, visitor: V) -> Result<(), Error>
where
    V: ComponentVisitor,
{
    let mut queue = VecDeque::new();

    queue.extend(component_ids);

    let arena = ComponentArena::instance();
    let mut futures = Vec::new();
    while let Some(comp_id) = queue.pop_front() {
        let func = async move { visit_component(comp_id, visitor) };
        futures.push(func);
        let comp = arena.get(comp_id).unwrap();
        for child_id in comp.children.iter() {
            queue.push_back(*child_id);
        }
    }

    match try_join_all(futures).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
