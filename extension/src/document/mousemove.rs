use crate::uitl::display_element_info_option;
use core::fmt::{Display, Formatter, Result as FmtResult};
use cubob::display_struct;

#[derive(Debug, Clone, Default)]
pub struct MousemoveElement {
    pub(super) disabled: bool,
    pub(super) old: Option<web_sys::Element>,
    pub(super) cur: Option<web_sys::Element>,
}

impl Display for MousemoveElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        display_struct(
            f,
            &[
                (&"disabled", &self.disabled),
                (&"old", &display_element_info_option(&self.old)),
                (&"cur", &display_element_info_option(&self.cur)),
            ],
        )
    }
}
