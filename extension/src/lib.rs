#![allow(non_snake_case)]
#![allow(unused)]
#![allow(dead_code)]
use dioxus::prelude::*;
use dioxus::web::Config;
use once_cell::sync::Lazy;
use tracing::{info, Level};
use wasm_bindgen::prelude::*;
pub mod ws;
mod global;
mod doc;
use global::*;

#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    // Init logger
    launch_run();
}
const SPIDER_BOX_ID:&'static str = "better-spider-box";
// static  LIS:Lazy<GlobalEventListener> = Lazy::new(|| GlobalEventListener::new());
pub fn launch_run(){
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    LaunchBuilder::web()
    .with_cfg(Config::new().rootname(SPIDER_BOX_ID))
    .launch(App);
}

fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
        }
    }
}