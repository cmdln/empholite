use anyhow::Result;
use yew::prelude::*;

pub(crate) struct InputString(pub(crate) String);

impl Into<Option<String>> for InputString {
    fn into(self) -> Option<String> {
        let InputString(value) = self;
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }
}

pub(crate) fn render_on_assign<T: PartialEq>(lhs: &mut T, mut rhs: T) -> Result<ShouldRender> {
    if lhs == &mut rhs {
        Ok(false)
    } else {
        *lhs = rhs;
        Ok(true)
    }
}

pub(crate) fn opt_render_on_assign<T, O>(lhs: &mut Option<T>, rhs: O) -> Result<ShouldRender>
where
    T: PartialEq,
    O: Into<Option<T>>,
{
    let mut rhs = rhs.into();
    if lhs == &mut rhs {
        Ok(false)
    } else {
        *lhs = rhs;
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wrap_empty_string() {
        let mut to_assign = Some(String::from("foo"));
        let value = String::default();
        let result = opt_render_on_assign(&mut to_assign, InputString(value));
        if let Ok(should_render) = result {
            assert!(should_render, "Should have indicated render is required");
            assert_eq!(None, to_assign);
        } else {
            panic!("Should have assigned without error!")
        }
    }

    #[test]
    fn test_wrap_string() {
        let mut to_assign = Some(String::from("foo"));
        let value = String::from("bar");
        let result = opt_render_on_assign(&mut to_assign, InputString(value));
        if let Ok(should_render) = result {
            assert!(should_render, "Should have indicated render is required");
            assert_eq!(Some("bar".to_owned()), to_assign);
        } else {
            panic!("Should have assigned without error!")
        }
    }

    #[test]
    fn test_wrap_string_unchanged() {
        let mut to_assign = Some(String::from("foo"));
        let value = String::from("foo");
        let result = opt_render_on_assign(&mut to_assign, InputString(value));
        if let Ok(should_render) = result {
            assert!(
                !should_render,
                "Should not have indicated render is required"
            );
            assert_eq!(Some("foo".to_owned()), to_assign);
        } else {
            panic!("Should have assigned without error!")
        }
    }
}
