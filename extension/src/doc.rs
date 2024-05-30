use dioxus::html::geometry::euclid::vec2;
use dioxus::html::mo;
use dioxus::prelude::*;
use gloo::events::EventListener;
use std::collections::{btree_map::Entry, BTreeMap};
use std::sync::Arc;
use tracing::{debug, info, warn};
use wasm_bindgen::prelude::*;
use web_sys::{Element, Event, *};

use crate::uitl::{element_xpath, elements_common_xpath};
use crate::{ActionMsg, HIGHLIGHT_CLASS, SELECTED_CLASS, SPIDER_BOX_ID};

pub fn init_document_events(coroutine: Coroutine<ActionMsg>) {
    init_document_mousemove_event();
    init_document_mouseup_event(coroutine);
}

fn init_document_mousemove_event() {
    let window = web_sys::window().expect("should have a window in this context");
    let doc_ = window.document().expect("window should have a document");
    let doc_clone = doc_.clone();
    // let mut current_element_xpath = "".to_string();
    let mut cur_element = None;
    let mut old_element = None;
    let doc_listener = EventListener::new(&doc_, "mousemove", move |event| {
        debug!("start element EventListener about mousemove");
        if let Some(mouse_element) = get_element_from_mouse_point(&doc_clone, event) {
            if let Some(cur) = &cur_element {
                if mouse_element == *cur {
                    debug!("mouse element is the same with the current element!");
                    return;
                }
                old_element = Some(cur.clone());
            }
            if let Some(old) = &old_element {
                remove_highlight(&old);
            }
            add_highlight(&mouse_element);
            cur_element = Some(mouse_element.clone());
            debug!("changed current element!{mouse_element:?}")
        }
    });
    let doc_box = Box::new(doc_listener);
    Box::leak(doc_box);
}

fn init_document_mouseup_event(coroutine: Coroutine<ActionMsg>) {
    let window = web_sys::window().expect("should have a window in this context");
    let doc_ = window.document().expect("window should have a document");
    let doc_clone = doc_.clone();
    // let mut current_element_xpath = "".to_string();
    let mut selected_elements: BTreeMap<Arc<String>, Element> = BTreeMap::new();
    let doc_listener = EventListener::new(&doc_, "mouseup", move |event| {
        let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
        // only the right button  handles this function
        // 0：主按键，通常指鼠标左键或默认值（译者注：如 document.getElementById('a').click() 这样触发就会是默认值）
        // 1：辅助按键，通常指鼠标滚轮中键
        // 2：次按键，通常指鼠标右键
        // https://developer.mozilla.org/zh-CN/docs/Web/API/MouseEvent/button
        if event.button() != 2 {
            return;
        }
        debug!("start element EventListener about mouseup of the right mouse button");
        if let Some(mouse_element) = get_element_from_mouse_point(&doc_clone, event) {
            let xpath = Arc::new(element_xpath(mouse_element.clone()));
            let old_len = selected_elements.len();

            match selected_elements.entry(xpath.clone()) {
                Entry::Vacant(v) => {
                    debug!("selected current element!{mouse_element:?}");
                    // handle related class
                    remove_highlight(&mouse_element);
                    add_selected(&mouse_element);
                    v.insert(mouse_element);
                }
                Entry::Occupied(o) => {
                    let v = o.remove();
                    // remove the selected class
                    remove_selected(&v);
                }
            }
            if selected_elements.len() > 2 {
                // ensure only owns at most 2 elements
                let (_, v) = selected_elements.pop_first().unwrap();
                remove_selected(&v);
            }
            let mut selected_xpaths = Box::new([
                selected_elements.first_entry().unwrap().key().clone(),
                selected_elements.last_entry().unwrap().key().clone(),
            ]);
            coroutine.send(ActionMsg::SelectedFromMouseupEvent(selected_xpaths))
        }
    });
    let doc_box = Box::new(doc_listener);
    Box::leak(doc_box);
}

fn get_element_from_mouse_point(doc: &Document, e: &Event) -> Option<Element> {
    let event = e.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
    let spider_box = doc
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
        return None;
    }
    doc.element_from_point(x as f32, y as f32)
}

fn remove_highlight(element: &Element) {
    element
        .class_list()
        .remove_2(HIGHLIGHT_CLASS, "hvr-glow")
        .unwrap_or_default();
}

fn add_highlight(element: &Element) {
    element
        .class_list()
        .add_2(HIGHLIGHT_CLASS, "hvr-glow")
        .unwrap_or_default();
}

fn add_selected(element: &Element) {
    element
        .class_list()
        .add_1(
            SELECTED_CLASS,
            // "animate__pulse",
            // "animate__infinite",
            // "animate__slower",
        )
        .unwrap_or_default();
}

fn remove_selected(element: &Element) {
    element
        .class_list()
        .remove_1(
            SELECTED_CLASS,
            // "animate__pulse",
            // "animate__infinite",
            // "animate__slower",
        )
        .unwrap_or_default();
}

pub fn handle_select_nodes(xpaths: &[Arc<String>], is_new: bool) {
    let window = web_sys::window().expect("should have a window in this context");
    let doc_ = window.document().expect("window should have a document");
    match elements_common_xpath(xpaths) {
        Ok(xpath) => match doc_.evaluate_with_opt_callback_and_type(&xpath, &doc_, None, 5) {
            Ok(iter_res) => {
                while let Ok(n) = iter_res.iterate_next() {
                    if let Some(n) = n {
                        // fixme:
                        match n.dyn_into::<web_sys::Element>() {
                            Ok(ele) => {
                                if is_new {
                                    add_selected(&ele);
                                } else {
                                    remove_selected(&ele);
                                }
                            }
                            Err(n) => match n.dyn_into::<web_sys::HtmlElement>() {
                                Ok(ele) => {
                                    if is_new {
                                        add_selected(&ele);
                                    } else {
                                        remove_selected(&ele);
                                    }
                                }
                                Err(e) => {
                                    tracing::error!("{}-{}-{:?}", e.node_name(), e.node_type(), e.node_value());
                                }
                            },
                        }
                    } else {
                        break;
                    }
                }
            }
            Err(e) => {
                tracing::error!("{e:?}");
            }
        },
        Err(e) => {
            tracing::error!("{e:?}")
        }
    }
}
