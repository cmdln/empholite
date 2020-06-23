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
