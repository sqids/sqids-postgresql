use pgrx::VariadicArray;

use crate::error::{Error, PgError};

pub fn process_min_length(min_length: i16) -> Result<u8, PgError> {
	if !(0..=255).contains(&min_length) {
		return Err(PgError::CustomError(Error::MinLengthRange));
	}

	Ok(min_length as u8)
}

pub fn process_numbers(numbers: VariadicArray<i64>) -> Result<Vec<u64>, PgError> {
	numbers
		.into_iter()
		.map(|n| match n.unwrap_or(0) {
			n if n < 0 => Err(PgError::CustomError(Error::NegativeNumbers)),
			n => Ok(n as u64),
		})
		.collect()
}
