use crate::uitl::*;
use core::fmt::{Display, Formatter, Result as FmtResult};
use std::thread::sleep;
use cubob::{display_list, display_list_from_iter, display_struct};
use std::collections::LinkedList;
use std::f64::consts::E;
use tracing::info;
use wasm_bindgen::JsCast;

#[derive(Debug, Clone, Default)]
pub(super) struct MouseupElement(LinkedList<web_sys::Element>);

impl MouseupElement {
    const MAX_ELEMENT_CNT: usize = 2;
    pub(super) fn set_element(&mut self, element: &web_sys::Element) {
        if self.0.len() >= Self::MAX_ELEMENT_CNT {
            if let Some(ele) = self.0.pop_front() {
                remove_selected(&ele);
            }
        }
        add_selected(&element);
        self.0.push_back(element.clone());
    }

    pub(super) fn contains(&self, element:&web_sys::Element) -> bool{
        self.0.contains(element)
    }

    fn common_xpath(&self) -> Vec<String> {
        self.0
            .iter()
            .map(|o| element_xpath(&o))
            .collect::<Vec<_>>()
    }
    fn get_all_elements_by_xpath(
        &self,
        xpath: &str,
        doc: &web_sys::Document,
    ) -> LinkedList<SelectedElement> {
        let mut ret = LinkedList::new();
        match doc.evaluate_with_opt_callback_and_type(&xpath, &doc, None, 5) {
            Ok(iter_res) => {
                while let Ok(n) = iter_res.iterate_next() {
                    if let Some(n) = n {
                        // fixme:
                        match n.dyn_into::<web_sys::Element>() {
                            Ok(element) => {
                                ret.push_back(SelectedElement {
                                    element,
                                    auto: false,
                                });
                            }
                            Err(e) => {
                                tracing::error!(
                                    "{}-{}-{:?}",
                                    e.node_name(),
                                    e.node_type(),
                                    e.node_value()
                                );
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
            Err(e) => {
                tracing::error!("{e:?}");
            }
        }
        ret
    }
    pub(super) fn selected_elements(&self, doc: &web_sys::Document) -> MouseupSelectedElement {
        let mut ret = MouseupSelectedElement::default();
        match elements_common_xpath(&self.common_xpath()) {
            Ok(common_xpath) => {
                let cnt = self.0.len();
                info!("elements common_xpath【{cnt},{self}】:{common_xpath}");
                ret.ready = self.get_all_elements_by_xpath(&common_xpath, doc);
                ret.common_xpath = common_xpath;
            }
            Err(e) => {
                tracing::error!("selected_elements:{e:?}");
            }
        }
        ret
    }
}

impl Display for MouseupElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_list_from_iter(f, self.0.iter().map(|e| display_element_info(&e)))
    }
}

#[derive(Debug, Clone)]
pub(super) struct SelectedElement {
    pub(super) element: web_sys::Element,
    pub(super) auto: bool,
}

impl Display for SelectedElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_struct(
            f,
            &[
                (&"element", &display_element_info(&self.element)),
                (&"auto", &self.auto),
            ],
        )
    }
}

#[derive(Debug, Clone, Default)]
pub(super) struct MouseupSelectedElement {
    pub(super) common_xpath: String,
    pub(super) ready: LinkedList<SelectedElement>,
}

impl Display for MouseupSelectedElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_struct(
            f,
            &[
                (&"common_xpath", &self.common_xpath),
                (
                    &"ready",
                    &self
                        .ready
                        .iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ],
        )
    }
}