#![feature(try_trait_v2)]

use std::ops::{FromResidual, Try};

#[derive(Clone, Copy, PartialEq, PartialOrd, Eq, Ord, Debug, Hash)]
pub struct HatchResult<T, E>(pub Result<T, E>);

impl<T, E> FromResidual<HatchResult<T, E>> for HatchResult<T, E> {
    fn from_residual(residual: HatchResult<T, E>) -> Self {
        residual
    }
}

impl<T, E> FromResidual<HatchResult<T, E>> for Result<T, E> {
    fn from_residual(residual: HatchResult<T, E>) -> Self {
        match residual {
            HatchResult(Ok(r)) => Ok(r),
            _ => unreachable!("E cannot be instantiated"),
        }
    }
}

impl<T, E> Try for HatchResult<T, E> {
    type Output = E;
    type Residual = HatchResult<T, E>;

    fn from_output(output: Self::Output) -> Self {
        HatchResult(Err(output))
    }

    fn branch(self) -> std::ops::ControlFlow<Self::Residual, Self::Output> {
        match self.0 {
            Ok(value) => std::ops::ControlFlow::Break(HatchResult(Ok(value))),
            Err(err_msg) => std::ops::ControlFlow::Continue(err_msg),
        }
    }
}

impl<T, E> HatchResult<T, E> {
    pub fn map<P>(self, construction_function: fn(T) -> P) -> HatchResult<P, E> {
        match self.0 {
            Ok(result) => HatchResult(Ok(construction_function(result))),
            Err(err) => HatchResult::<P, E>(Err(err)),
        }
    }
}

pub trait EscapeHatch<T, E> {
    fn hatch(self) -> HatchResult<T, E>;
}

impl<T, E> EscapeHatch<T, E> for Result<T, E> {
    fn hatch(self) -> HatchResult<T, E> {
        HatchResult(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::EscapeHatch;

    use super::HatchResult;

    fn dummy_fn<T, E: Default>(r: Result<T, E>) -> Result<T, E> {
        let err = HatchResult(r)?;
        Err(err)
    }

    #[test]
    fn hatch_implementation() {
        let r1 = HatchResult::<u32, String>(Ok(3));
        let r2 = Result::<u32, String>::Ok(3).hatch();
        assert_eq!(r1, r2);
    }

    #[test]
    fn escapes_when_inner_result_is_ok() {
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
