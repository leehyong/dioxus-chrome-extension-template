#![allow(non_snake_case)]
#![allow(dead_code)]

use dioxus::prelude::*;
use gloo::events::EventListener;
use tracing::info;
use wasm_bindgen::prelude::*;
use web_sys::{Event, HtmlElement, MouseEvent};

use crate::{GLOBAL_CONFIG, GLOBAL_DATA};

fn doc_mousemove_event() {
    let window = web_sys::window().expect("should have a window in this context");
    let doc_ = window.document().expect("window should have a document");
    let doc_clone = doc_.clone();

    let doc_listener = EventListener::new(&doc_, "mousemove", move |event| {
        let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        let spider_box = doc_clone
            .get_element_by_id(crate::SPIDER_BOX_ID)
            .expect(&format!("should have a '{}' of div ", crate::SPIDER_BOX_ID));
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
            let mut conf = GLOBAL_CONFIG.lock().unwrap();
            conf.is_toolbox = true;
            info!("mouse in the spider box");
            return;
        }
        if let Some(mouse_element) = doc_clone.element_from_point(x as f32, y as f32) {
            let mut cur_ele = None;
            let data = GLOBAL_DATA.lock().unwrap();
            if let Some(cur) = &data.current {
                cur_ele = Some(cur.clone());
                if mouse_element == *cur {
                    info!("mouse element is the same with the current element!");
                    return;
                } else {
                    // remove related css class
                    if let Some(old_ele) = &data.old {
                        let conf = GLOBAL_CONFIG.lock().unwrap();
                        old_ele
                            .class_list()
                            .remove_1(&conf.highlight)
                            .unwrap_or_default();
                    }
                }
            }
            {
                let conf = GLOBAL_CONFIG.lock().unwrap();
                mouse_element
                    .class_list()
                    .add_1(&conf.highlight)
                    .unwrap_or_default();
            }
            let mut data = GLOBAL_DATA.lock().unwrap();
            data.old = cur_ele;
            data.current = Some(mouse_element.clone());
        }
    });
    let doc_listener_box = Box::new(doc_listener);
    // leak the global event lister to avoiding drop.
    let _ = Box::leak(doc_listener_box);
}

// pub fn use_root_document_events() -> Arc<GlobalEventListener> {
//     // use context provider to avoid EventListener to drop !
//     use_root_context(GlobalEventListener::new)
// }
