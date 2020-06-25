use anyhow::Result;
use yew::prelude::*;

pub(crate) fn render_on_assign<T: PartialEq>(lhs: &mut T, mut rhs: T) -> Result<ShouldRender> {
    if lhs == &mut rhs {
        Ok(false)
    } else {
        *lhs = rhs;
        Ok(true)
    }
}

pub(crate) fn opt_render_on_assign(lhs: &mut Option<String>, rhs: String) -> Result<ShouldRender> {
    if lhs.as_ref().map(|lhs| lhs == &rhs).unwrap_or_default() {
        Ok(false)
    } else if rhs.is_empty() {
        *lhs = None;
        Ok(true)
    } else {
        *lhs = Some(rhs);
        Ok(true)
    }
}
