use std::any::type_name;

use anyhow::{Context, Result};

pub trait OptionExt<T> {
    fn ok(self) -> Result<T>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok(self) -> Result<T> {
        self.context(format!("Option<{}> is None", type_name::<T>()))
    }
}

#[cfg(test)]
mod tests {
    use crate::option_ext::OptionExt;

    #[test]
    fn test() {
        assert_eq!(
            None::<i32>.ok().unwrap_err().to_string(),
            "Option<i32> is None"
        );
    }
}
