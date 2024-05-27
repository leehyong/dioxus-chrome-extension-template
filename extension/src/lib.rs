#![allow(non_snake_case)]
use dioxus::prelude::*;
use dioxus::web::Config;
use tracing::{info, Level};
use wasm_bindgen::prelude::*;
pub mod ws;
pub mod global;
mod doc;
use global::*;

use crate::doc::GlobalEventListener;
#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    // Init logger
    launch_run();
}
const spider_box_id:&'static str = "better-spider-box";
pub fn launch_run(){
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    LaunchBuilder::web()
    .with_cfg(Config::new().rootname(spider_box_id))
    .launch(App);
}

fn App() -> Element {
    let _listener = GlobalEventListener::new();
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}