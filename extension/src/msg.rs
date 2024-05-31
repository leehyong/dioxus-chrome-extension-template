use crate::SelectedXpathBox;
use std::sync::Arc;
#[derive(Debug, Clone, Copy)]
pub enum ActionMsg {
    SelectAllRelated,

    ClearSelectAllRelated,
    ToggleEnableMousemove,
}
