#![feature(try_trait_v2, try_trait_v2_residual)]

use std::{
    convert::Infallible,
    ops::{ControlFlow, FromResidual, Residual, Try},
    panic::Location,
};

pub struct TracedResult<T, E> {
    pub result: Result<T, E>,
    pub trace: Vec<&'static Location<'static>>,
}

impl<T, E> TracedResult<T, E> {
    pub fn from_output(value: T) -> Self {
        Self {
            result: Ok(value),
            trace: Vec::new(),
        }
    }

    pub fn from_error(error: E) -> Self {
        Self {
            result: Err(error),
            trace: vec![Location::caller()],
        }
    }

    pub fn from_result(result: Result<T, E>) -> Self {
        match result {
            Ok(value) => Self::from_output(value),
            Err(error) => Self::from_error(error),
        }
    }

    pub fn handle(self) -> Result<T, (E, Vec<&'static Location<'static>>)> {
        match self.result {
            Ok(value) => Ok(value),
            Err(error) => Err((error, self.trace)),
        }
    }
}

impl<T, E> Try for TracedResult<T, E> {
    type Output = T;
    type Residual = TracedResult<Infallible, E>;

    fn from_output(value: T) -> Self {
        Self {
            result: Ok(value),
            trace: Vec::new(),
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, T> {
        match self.result {
            Ok(value) => ControlFlow::Continue(value),
            Err(error) => ControlFlow::Break(TracedResult {
                result: Err(error),
                trace: self.trace,
            }),
        }
    }
}

impl<T, E> Residual<T> for TracedResult<Infallible, E> {
    type TryType = TracedResult<T, E>;
}

impl<T, E> FromResidual<TracedResult<Infallible, E>> for TracedResult<T, E> {
    #[track_caller]
    fn from_residual(residual: TracedResult<Infallible, E>) -> Self {
        let mut trace = residual.trace;
        trace.push(Location::caller());

        Self {
            result: residual.result.map(|never| match never {}),
            trace,
        }
    }
}

impl<T, E> FromResidual<Result<Infallible, E>> for TracedResult<T, E> {
    #[track_caller]
    fn from_residual(residual: Result<Infallible, E>) -> Self {
        Self {
            result: residual.map(|never| match never {}),
            trace: vec![Location::caller()],
        }
    }
}
