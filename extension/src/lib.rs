#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use std::borrow::Borrow;
use std::sync::Arc;

use dioxus::html::mo;
use dioxus::prelude::*;
use dioxus::signals::Readable;
use dioxus::web::Config;
use futures::StreamExt;
use gloo::events::EventListener;
use tracing::{debug, info, warn, Level};
use wasm_bindgen::prelude::*;

mod document;
mod error;
mod global;
mod msg;
mod uitl;
pub mod ws;
use crate::uitl::element_xpath;
use document::init_spider_document_events;
use global::*;
pub use msg::ActionMsg;
use web_sys::HtmlElement;

type SelectedXpathBox = Box<[Arc<String>; 2]>;
#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    // Init logger
    launch_run();
}
const SPIDER_BOX_ID: &'static str = "better-spider-box";
const HIGHLIGHT_CLASS: &'static str = "better-spider-highlight";
const SELECTED_CLASS: &'static str = "better-spider-selected";

// static  LIS:Lazy<GlobalEventListener> = Lazy::new(|| GlobalEventListener::new());
pub fn launch_run() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    LaunchBuilder::web()
        .with_cfg(Config::new().rootname(SPIDER_BOX_ID))
        .launch(App);
}
// static current_element_xpath: GlobalSignal<String> = Signal::global(|| "".to_string());
// static old_element_xpath: GlobalSignal<String> = Signal::global(|| "".to_string());

fn App() -> Element {
    let msg_sender = init_spider_document_events();
    // init_document_events(msg_sender);
    let mut count = use_signal(|| 0);
    rsx! {
        div {
            // onmounted: move|_|{info!("{current_element_xpath}-{old_element_xpath}");},
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }

            button { onclick: move |_| msg_sender.send(ActionMsg::SelectAllRelated), "select all related" }
            button { onclick: move |_| msg_sender.send(ActionMsg::ClearSelectAllRelated), "clear selected" }
            button { onclick: move |_| msg_sender.send(ActionMsg::ToggleEnableMousemove), "toggle mousemove" }

        }
    }
}
