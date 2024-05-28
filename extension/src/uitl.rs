use web_sys::Element;

pub fn element_xpath(node: Element) -> String {
    let mut ret = Vec::with_capacity(20);
    fn _element_xpath(node: Element, ret: &mut Vec<String>) {
        if let Some(parent) = node.parent_element() {
            _element_xpath(parent, ret);
        }
        let mut cur = node.clone();
        let mut i = 1;
        while let Some(n) = cur.previous_element_sibling() {
            if n .tag_name() == node.tag_name() {
                i += 1;
            }
            cur = n;
        }
        if i == 1 {
            ret.push(format!("{}", node.tag_name()));
        } else {
            ret.push(format!("{}[{}]", node.tag_name(),i));
        }
    }
    _element_xpath(node, &mut ret);
    format!("/{}", ret.join("/").to_lowercase())
}
