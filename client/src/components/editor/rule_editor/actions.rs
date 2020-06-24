use super::RuleEditor;
use crate::RuleType;
use anyhow::{bail, Result};
use yew::{prelude::*, web_sys::HtmlSelectElement};

impl RuleEditor {
    pub(super) fn handle_type(&mut self, selected: HtmlSelectElement) -> Result<ShouldRender> {
        self.state.rule_type = Some(match selected.selected_index() {
            1 => RuleType::Authenticated,
            2 => RuleType::Subject,
            _ => bail!("Invalid selection for rule type!"),
        });
        Ok(true)
    }

    pub(super) fn handle_remove(&self) -> Result<ShouldRender> {
        self.props.on_remove.emit(());
        Ok(false)
    }
}
