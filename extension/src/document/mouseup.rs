use crate::uitl::*;
use core::fmt::{Display, Formatter, Result as FmtResult};
use cubob::{display_list, display_list_from_iter, display_struct};
use std::collections::{LinkedList, VecDeque};
use std::f64::consts::E;
use std::thread::sleep;
use tracing::info;
use wasm_bindgen::JsCast;
const MAX_ELEMENT_CNT: usize = 2;

#[derive(Debug, Clone, Default)]
pub(super) struct MouseupElement(VecDeque<web_sys::Element>);

impl MouseupElement {
    pub(super) fn toggle_one_element(&mut self, element: &web_sys::Element) {
        let idx = self.0.iter().position(|o| o == element);
        if let Some(idx) = idx {
            // find the element, and delete it, remove selected css class
            self.0.remove(idx);
            remove_selected(&element);
            return;
        }

        if self.0.len() >= MAX_ELEMENT_CNT {
            if let Some(ele) = self.0.pop_front() {
                remove_selected(&ele);
            }
        }
        add_selected(&element);
        self.0.push_back(element.clone());
    }

    pub(super) fn contains(&self, element: &web_sys::Element) -> bool {
        self.0.contains(element)
    }

    fn elements_xpath(&self) -> Vec<String> {
        self.0.iter().map(|o| element_xpath(&o)).collect::<Vec<_>>()
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
    pub(super) fn toggle_related_elements(
        &self,
        doc: &web_sys::Document,
    ) -> MouseupSelectedElement {
        let mut ret = MouseupSelectedElement::default();
        match elements_common_xpath(&self.elements_xpath()) {
            Ok(common_xpath) => {
                let cnt = self.0.len();
                info!("elements common_xpath【{cnt},{self}】:{common_xpath}");
                ret.nodes = self.get_all_elements_by_xpath(&common_xpath, doc);
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
    pub(super) nodes: LinkedList<SelectedElement>,
}

impl MouseupSelectedElement {
    pub(super)fn add_nodes_selected(&self){
        self.nodes.iter().for_each(|n|{
            add_selected(&n.element);
        });
    }
    
    pub(super)fn remove_nodes_selected(&self){
        self.nodes.iter().for_each(|n|{
            remove_selected(&n.element);
        });
    }
}

impl Display for MouseupSelectedElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_struct(
            f,
            &[
                (&"common_xpath", &self.common_xpath),
                (
                    &"nodes",
                    &self
                        .nodes
                        .iter()
                        .map(|o| o.to_string())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            ],
        )
    }
}
