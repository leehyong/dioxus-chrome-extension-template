use std::collections::LinkedList;
use std::sync::Arc;

use crate::error::{SpiderError, SpiderResult};
use crate::{HIGHLIGHT_CLASS, SELECTED_CLASS, SPIDER_BOX_ID};
use once_cell::sync::Lazy;
use regex::Regex;
use std::borrow::Cow;
use tracing::debug;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{Document, Element, Event, HtmlElement};

static XPATH_INDEX_REPLACE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\[\d+\]").unwrap());
const INGORES_ELEMENTS: [&'static str; 3] = ["HTML", "BODY", "HEAD"];
pub fn element_xpath(node: &Element) -> String {
    let mut ret = Vec::with_capacity(20);
    fn _element_xpath(node: &Element, ret: &mut Vec<String>) {
        if let Some(parent) = node.parent_element() {
            _element_xpath(&parent, ret);
        }
        if INGORES_ELEMENTS.contains(&node.tag_name().as_str()) {
            ret.push(format!("{}", node.tag_name()));
            return;
        }
        let mut cur = node.clone();
        let mut i = 1;
        while let Some(n) = cur.previous_element_sibling() {
            if n.tag_name() == node.tag_name() {
                i += 1;
            }
            cur = n;
        }
        ret.push(format!("{}[{}]", node.tag_name(), i));
    }
    _element_xpath(node, &mut ret);
    format!("/{}", ret.join("/").to_lowercase())
}

pub fn elements_common_xpath<T: AsRef<str>>(
    xpaths: &[T],
    doc: &web_sys::Document,
) -> SpiderResult<String> {
    match xpaths.len() {
        0 => Ok("".to_string()),
        1 => {
            let xpath0 = &xpaths[0].as_ref();
            let mut start = xpath0.len() - 1;
            let mut end = start;
            let mut ret = "".to_string();
            for (i, c) in xpath0.chars().rev().enumerate() {
                match c {
                    ']' => end = i,
                    '[' => start = i,
                    _ => {}
                }
                // because of the rev operation, the end index must be less than start
                if end < start {
                    // we need reindex from the begin to the end;
                    let _start = xpath0.len() - start;
                    let left = &xpath0[.._start];
                    let right = &xpath0[_start..];
                    // find a xpath that can get more than one elements
                    if let Cow::Owned(p) = XPATH_INDEX_REPLACE_RE.replacen(&right, 1, "") {
                        let xp = format!("{left}{p}");
                        let elements = get_all_elements_by_xpath(&xp, doc);
                        if elements.len() < 2 {
                            continue;
                        }
                        ret = xp.to_owned();
                        break;
                    }
                }
            }
            return Ok(ret.to_owned());
        }
        2 => {
            let xpath0 = &xpaths[0].as_ref();
            let xpath1 = &xpaths[1].as_ref();
            let cnt0 = xpath0.rsplit("/").into_iter().count();
            let cnt1 = xpath1.rsplit("/").into_iter().count();
            if cnt0 != cnt1 {
                return Ok("".to_string());
            }
            let mut ret = Vec::with_capacity(cnt0);
            for (x0, x1) in xpath0
                .rsplit("/")
                .into_iter()
                .zip(xpath1.rsplit("/").into_iter())
            {
                if x0 == x1 {
                    ret.push(std::borrow::Cow::Borrowed(x0));
                } else {
                    let _x0 = XPATH_INDEX_REPLACE_RE.replace_all(&*x0, "");
                    let _x1 = XPATH_INDEX_REPLACE_RE.replace_all(&*x1, "");
                    if _x0 == _x1 {
                        ret.push(_x0);
                    } else {
                        return Err(SpiderError::NotTheSameCategoryXpath(format!(
                            "{_x0}!={_x1}"
                        )));
                    }
                }
            }
            Ok(format!(
                "/{}",
                ret.iter()
                    .rev()
                    .map(|o| o.as_ref())
                    .collect::<Vec<_>>()
                    .join("/")
            ))
        }
        _ => unimplemented!(),
    }
}
pub fn display_element_info(node: &Element) -> String {
    format!("{}[xpath:{}]", node.tag_name(), element_xpath(node))
}

pub fn display_element_info_option(node: &Option<Element>) -> String {
    match node {
        Some(ref node) => {
            format!("{}[xpath:{}]", node.tag_name(), element_xpath(node))
        }
        None => "".to_string(),
    }
}

pub fn remove_highlight(element: &Element) {
    element
        .class_list()
        .remove_2(HIGHLIGHT_CLASS, "hvr-glow")
        .unwrap_or_default();
}

pub fn add_highlight(element: &Element) {
    element
        .class_list()
        .add_2(HIGHLIGHT_CLASS, "hvr-glow")
        .unwrap_or_default();
}

pub fn add_selected(element: &Element) {
    element
        .class_list()
        .add_1(SELECTED_CLASS)
        .unwrap_or_default();
}

pub fn remove_selected(element: &Element) {
    element
        .class_list()
        .remove_1(SELECTED_CLASS)
        .unwrap_or_default();
}

pub fn contains_selected(element: &Element) -> bool {
    element.class_list().contains(SELECTED_CLASS)
}

pub fn get_all_elements_by_xpath(
    xpath: &str,
    doc: &web_sys::Document,
) -> LinkedList<web_sys::Element> {
    let mut ret = LinkedList::new();
    match doc.evaluate_with_opt_callback_and_type(xpath, doc, None, 5) {
        Ok(iter_res) => {
            while let Ok(n) = iter_res.iterate_next() {
                if let Some(n) = n {
                    // fixme:
                    match n.dyn_into::<web_sys::Element>() {
                        Ok(element) => {
                            ret.push_back(element);
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
