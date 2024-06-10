use std::{
    any::type_name,
    error::Error,
    fmt::{Debug, Display, Formatter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoneValueError<T> {
    val: std::marker::PhantomData<T>,
}

impl<T> Display for NoneValueError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Option<{}> is None", type_name::<T>())
    }
}

impl<T: Debug> Error for NoneValueError<T> {}

pub trait OptionExt<T> {
    fn ok(self) -> Result<T, NoneValueError<T>>;
}

impl<T> OptionExt<T> for Option<T> {
    fn ok(self) -> Result<T, NoneValueError<T>> {
        self.ok_or(NoneValueError {
            val: std::marker::PhantomData,
        })
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
