use pgrx::prelude::*;
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum Error {
	#[error("Min length has to be between 0 and 255")]
	MinLengthRange,
	#[error("Numbers cannot be negative")]
	NegativeNumbers,
}

pub enum PgError {
	SqidsError(sqids::Error),
	CustomError(Error),
}

impl fmt::Display for PgError {
	fn fmt(&self, _f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			PgError::SqidsError(err) => error!("{}", err),
			PgError::CustomError(err) => error!("{}", err),
		}
	}
}

impl From<sqids::Error> for PgError {
	fn from(err: sqids::Error) -> Self {
		PgError::SqidsError(err)
	}
}
