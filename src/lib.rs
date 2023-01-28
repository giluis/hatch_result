#![feature(try_trait_v2)]

use std::ops::{FromResidual, Try};

pub struct DisjunctResultWrapper<T, E>(pub Result<T, E>);

impl<T, E> FromResidual<DisjunctResultWrapper<T, E>> for DisjunctResultWrapper<T, E> {
    fn from_residual(residual: DisjunctResultWrapper<T, E>) -> Self {
        residual
    }
}

impl<T, E> FromResidual<DisjunctResultWrapper<T, E>> for Result<T, E> {
    fn from_residual(residual: DisjunctResultWrapper<T, E>) -> Self {
        match residual {
            DisjunctResultWrapper(Ok(r)) => Ok(r),
            _ => unreachable!("E cannot be instantiated"),
        }
    }
}

impl<T, E> Try for DisjunctResultWrapper<T, E> {
    type Output = E;
    type Residual = DisjunctResultWrapper<T, E>;

    fn from_output(output: Self::Output) -> Self {
        DisjunctResultWrapper(Err(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(value) => std::ops::ControlFlow::Break(DisjunctResultWrapper(Ok(value))),
            Err(err_msg) => std::ops::ControlFlow::Continue(err_msg),
        }
    }
}

impl<T, E> DisjunctResultWrapper<T, E> {
    pub fn map<P>(self, construction_function: fn(T) -> P) -> DisjunctResultWrapper<P, E> {
        match self.0 {
            Ok(result) => DisjunctResultWrapper(Ok(construction_function(result))),
            Err(err) => DisjunctResultWrapper::<P, E>(Err(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DisjunctResultWrapper;

    fn dummy_fn<T, E: Default>(r: Result<T, E>) -> Result<T, E> {
        let err = DisjunctResultWrapper(r)?;
        Err(err)
    }

    #[test]
    fn test_it_breaks_when_inner_result_is_ok() {
        let input = Result::<u32, String>::Ok(3);
        let result = dummy_fn(input.clone());
        assert!(input == result);
    }

    #[test]
    fn test_it_continues_when_inner_result_is_err() {
        let input = Result::<u32, String>::Err("Some error msg".to_string());
        let result = dummy_fn(input);
        assert!(result.is_err());
    }
}
