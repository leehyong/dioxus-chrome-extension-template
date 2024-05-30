use std::sync::Arc;
use crate::SelectedXpathBox;
#[derive(Debug)]
pub enum ActionMsg {
    SelectedFromMouseupEvent(SelectedXpathBox),
    SelectAllRelated,

    ClearSelectAllRelated,
}
