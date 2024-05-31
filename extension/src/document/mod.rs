use crate::ActionMsg;
use crate::{uitl::*, SPIDER_BOX_ID};
use dioxus::prelude::*;
use futures::channel::mpsc::UnboundedReceiver;
use futures::StreamExt;
use gloo::events::EventListener;
use once_cell::sync::Lazy;
use web_sys::{Document, Element, Event, HtmlElement};

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use wasm_bindgen_futures::spawn_local;
mod mousemove;
mod mouseup;
use self::mousemove::MousemoveElement;
use self::mouseup::MouseupElement;

static SPIDER_DOCUMENT: Lazy<BetterSpiderDocument> = Lazy::new(BetterSpiderDocument::default);

type MousemoveElementRcRwlock = Rc<RwLock<MousemoveElement>>;
type MouseupElementRcRwlock = Rc<RwLock<MouseupElement>>;

#[derive(Debug, Clone, Copy, Default)]
struct BetterSpiderDocument;

impl BetterSpiderDocument {
    fn init(self) -> Coroutine<ActionMsg> {
        use_coroutine(|mut rx| async move {
            let mousemove_element = MousemoveElementRcRwlock::default();
            let mouseup_element = MouseupElementRcRwlock::default();
            self.init_mousemove_event(mousemove_element.clone());
            self.init_mouseup_event(mouseup_element.clone());
            self.handle_events(rx, mousemove_element.clone(), mouseup_element.clone());
        })
    }

    fn init_mousemove_event(self, mousemove_element: MousemoveElementRcRwlock) {
        let window = web_sys::window().expect("should have a window in this context");
        let doc = window.document().expect("window should have a document");
        let doc2 = doc.clone();
       
        let doc_listener = EventListener::new(&doc, "mousemove", move |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let x = event.client_x();
            let y = event.client_y();
            let doc2 = doc2.clone();
            let mousemove_element = mousemove_element.clone();
            spawn_local(async move {
                if mousemove_element.read().await.disabled {
                    debug!("disabled element EventListener about mousemove");
                    return;
                }
                debug!("start element EventListener about mousemove");
                if let Some(mouse_element) = get_element_from_mouse_point(&doc2, x,y) {
                    let mut move_ele = mousemove_element.write().await;
                    if let Some(cur) =&move_ele.cur {
                        if mouse_element == *cur {
                            debug!("mouse element is the same with the current element!");
                            return;
                        }
                        move_ele.old = Some(cur.clone());
                    }
                    if let Some(old) = &move_ele.old {
                        remove_highlight(&old);
                    }
                    add_highlight(&mouse_element);
                    move_ele.cur = Some(mouse_element.clone());
                    debug!("changed current element!{move_ele}")
                }
            })
        });
        doc_listener.forget();
    }

    fn init_mouseup_event(self, mouseup_element: MouseupElementRcRwlock) {
        let window = web_sys::window().expect("should have a window in this context");
        let doc1 = window.document().expect("window should have a document");
        let doc2 = doc1.clone(); 
        let doc_listener = EventListener::new(&doc1, "mouseup", move |event| {
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let x = event.client_x();
            let y = event.client_y();
            let button = event.button();
            if button != 2 {
                return;
            }
            let mouseup_element = mouseup_element.clone();
            let doc_ = doc2.clone();
            spawn_local(async move {
                // only the right button  handles this function
                // 0：主按键，通常指鼠标左键或默认值（译者注：如 document.getElementById('a').click() 这样触发就会是默认值）
                // 1：辅助按键，通常指鼠标滚轮中键
                // 2：次按键，通常指鼠标右键
                // https://developer.mozilla.org/zh-CN/docs/Web/API/MouseEvent/button
                debug!("start element EventListener about mouseup of the right mouse button");
                if let Some(mouse_element) = get_element_from_mouse_point(&doc_, x,y) {
                    mouseup_element.write().await.set_element(&mouse_element);
                }
            })
        });
        doc_listener.forget();
    }

    fn handle_events(
        self,
        mut rx: UnboundedReceiver<ActionMsg>,
        mousemove_element: MousemoveElementRcRwlock,
        mouseup_element: MouseupElementRcRwlock,
    ) {
        spawn_local(async move {
            while let Some(msg) = rx.next().await {
                match msg {
                    ActionMsg::SelectAllRelated => {
                        info!("received: SelectAllRelated");
                    }
                    ActionMsg::ClearSelectAllRelated => {
                        info!("received: ClearSelectAllRelated");
                    }
                    ActionMsg::ToggleEnableMousemove => {
                        let mut mousemove_element_ = mousemove_element.write().await;
                        mousemove_element_.disabled = !mousemove_element_.disabled;
                    }
                }
            }
        });
    }
}

pub fn init_spider_document_events() -> Coroutine<ActionMsg> {
    SPIDER_DOCUMENT.init()
}

pub fn get_element_from_mouse_point(doc: &Document, x:i32, y:i32) -> Option<Element> {
    let spider_box = doc
        .get_element_by_id(SPIDER_BOX_ID)
        .expect(&format!("should have a '{}' of div ", SPIDER_BOX_ID));
    let spider_box_html = spider_box.dyn_ref::<HtmlElement>().unwrap_throw();
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
