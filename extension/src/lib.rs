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
pub mod ws;
use global::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlElement;

use crate::uitl::element_xpath;

#[wasm_bindgen]
pub fn run() {
    console_error_panic_hook::set_once();
    // Init logger
    launch_run();
}
const SPIDER_BOX_ID: &'static str = "better-spider-box";
const HIGHLIGHT_CLASS: &'static str = "highlight";
const SELECTED_CLASS: &'static str = "selected";

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
    let cor = use_coroutine(move |rx| async {
        init_document_mousemove_event();
    });
    cor.send(1);
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

fn init_document_mousemove_event() {
    let window = web_sys::window().expect("should have a window in this context");
    let doc_ = window.document().expect("window should have a document");
    let doc_clone = doc_.clone();
    // let mut current_element_xpath = "".to_string();
    let mut cur_element = None;
    let mut old_element = None;
    debug!("start element coroutine");
    let doc_listener = EventListener::new(&doc_, "mousemove", move |event| {
        debug!("start element coroutine in mousemove");
        let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let spider_box = doc_clone
            .get_element_by_id(SPIDER_BOX_ID)
            .expect(&format!("should have a '{}' of div ", SPIDER_BOX_ID));
        let spider_box_html = spider_box.dyn_ref::<HtmlElement>().unwrap_throw();
        let x = event.client_x();
        let y = event.client_y();
        let x_box_offset_left = spider_box_html.offset_left();
        let y_box_offset_top = spider_box_html.offset_top();
        let box_offset_width = spider_box_html.offset_width();
        let box_offset_height = spider_box_html.offset_height();
        let x_max = x_box_offset_left + box_offset_width;
        let y_max = y_box_offset_top + box_offset_height;
        // check if the mouse moves into the spider boox
        if x >= x_box_offset_left && x <= x_max && y >= y_box_offset_top && y <= y_max {
            debug!("mouse in the spider box");
            return;
        }
        if let Some(mouse_element) = doc_clone.element_from_point(x as f32, y as f32) {
            if let Some(cur) = &cur_element {
                if mouse_element == *cur {
                    debug!("mouse element is the same with the current element!");
                    return;
                }
                old_element = Some(cur.clone());
            }
            if let Some(old) = &old_element {
                old.class_list()
                    .remove_1(HIGHLIGHT_CLASS)
                    .unwrap_or_default();
            }
            mouse_element
                .class_list()
                .add_1(HIGHLIGHT_CLASS)
                .unwrap_or_default();
            cur_element = Some(mouse_element.clone());
            debug!("changed current element!{mouse_element:?}")
        }
    });
    let doc_box = Box::new(doc_listener);
    Box::leak(doc_box);
}
