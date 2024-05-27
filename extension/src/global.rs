#![allow(dead_code)]
use dioxus::prelude::*;
use dioxus_logger::tracing::*;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Mutex;
use web_sys::Element;

static mut GLOBAL_DATA: Lazy<Mutex<GlobalData>> = Lazy::new(|| Mutex::new(GlobalData::default()));

static mut GLOBAL_IS_IN_SPIDER_BOX: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));

#[derive(Debug, Clone, Deserialize)]
pub struct GlobalConfig {
    pub highlight: String,
    pub selected: String,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            highlight: "highlight".to_string(),
            selected: "selected".to_string(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalData {
    pub old: Option<Element>,
    pub current: Option<Element>,
    pub task_id: String,
    pub task_step: u64,
    pub highlight: String,
    pub selected: String,
}
impl Default for GlobalData {
    fn default() -> Self {
        Self {
            old: Default::default(),
            current: Default::default(),
            task_id: Default::default(),
            task_step: Default::default(),
            highlight: "highlight".to_string(),
            selected: "selected".to_string(),
        }
    }
}

// pub fn use_global_context_provider() -> Signal<GlobalData> {
//     use_root_context(|| Signal::new(GlobalData::default()))
// }

// pub fn use_is_in_spider_box() -> Signal<bool> {
//     use_root_context(|| Signal::new(false))
// }

// pub fn use_global_config() -> GlobalConfig {
//     use_root_context(GlobalConfig::default)
// }
