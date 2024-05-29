#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use std::borrow::Borrow;

use dioxus::html::mo;
use dioxus::prelude::*;
use dioxus::signals::Readable;
use dioxus::web::Config;
use futures::StreamExt;
use gloo::events::EventListener;
use tracing::{debug, info, Level};
use wasm_bindgen::prelude::*;

mod global;
mod uitl;
mod doc;
pub mod ws;
use global::*;
use web_sys::HtmlElement;

use crate::uitl::element_xpath;
use crate::doc::init_document_events;

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
    init_document_events();

    let mut count = use_signal(|| 0);
    rsx! {
        div {
            // onmounted: move|_|{info!("{current_element_xpath}-{old_element_xpath}");},
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}

