#![allow(dead_code)]
use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use serde::Deserialize;
use std::sync::Mutex;
use web_sys::Element;

#[derive(Clone, Debug)]
pub struct GlobalData {
    pub old: Option<Element>,
    pub current: Option<Element>,
    pub task_id: String,
    pub task_step: u64,
}

impl Default for GlobalData {
    fn default() -> Self {
        Self {
            old: Default::default(),
            current: Default::default(),
            task_id: Default::default(),
            task_step: Default::default(),
        }
    }
}
