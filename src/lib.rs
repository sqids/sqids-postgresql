use pgrx::prelude::*;
use sqids::Sqids;

use error::PgError;

mod error;
mod utils;

::pgrx::pg_module_magic!();

#[pg_extern(name = "sqids_encode")]
fn sqids_encode(numbers: VariadicArray<i64>) -> Result<String, PgError> {
	let sqids = Sqids::builder().build().map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_alphabet(
	alphabet: &str,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_min_length(
	min_length: i16,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.min_length(utils::process_min_length(min_length)?)
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_blocklist(
	blocklist: Array<String>,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_alphabet_min_length(
	alphabet: &str,
	min_length: i16,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.min_length(utils::process_min_length(min_length)?)
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_alphabet_blocklist(
	alphabet: &str,
	blocklist: Array<String>,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_min_length_blocklist(
	min_length: i16,
	blocklist: Array<String>,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.min_length(utils::process_min_length(min_length)?)
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_encode")]
fn sqids_encode_with_alphabet_min_length_blocklist(
	alphabet: &str,
	min_length: i16,
	blocklist: Array<String>,
	numbers: VariadicArray<i64>,
) -> Result<String, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.min_length(utils::process_min_length(min_length)?)
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	sqids.encode(&utils::process_numbers(numbers)?).map_err(PgError::SqidsError)
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode(id: &str) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder().build().map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_alphabet(alphabet: &str, id: &str) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_min_length(min_length: i16, id: &str) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.min_length(utils::process_min_length(min_length)?)
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_blocklist(blocklist: Array<String>, id: &str) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_alphabet_min_length(
	alphabet: &str,
	min_length: i16,
	id: &str,
) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.min_length(utils::process_min_length(min_length)?)
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_alphabet_blocklist(
	alphabet: &str,
	blocklist: Array<String>,
	id: &str,
) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_min_length_blocklist(
	min_length: i16,
	blocklist: Array<String>,
	id: &str,
) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.min_length(utils::process_min_length(min_length)?)
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[pg_extern(name = "sqids_decode")]
fn sqids_decode_with_alphabet_min_length_blocklist(
	alphabet: &str,
	min_length: i16,
	blocklist: Array<String>,
	id: &str,
) -> Result<Vec<i64>, PgError> {
	let sqids = Sqids::builder()
		.alphabet(alphabet.chars().collect())
		.min_length(utils::process_min_length(min_length)?)
		.blocklist(blocklist.iter().flatten().collect())
		.build()
		.map_err(PgError::SqidsError)?;

	Ok(sqids.decode(id).iter().map(|n| *n as i64).collect())
}

#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
	use pgrx::prelude::*;
	use std::collections::HashMap;

	// @NOTE since this extension uses `https://crates.io/crates/sqids`, this repo does not include the standard Sqids unit tests (which are included
	// in the external Rust library). Therefore, we are testing the SQL functions only.

	#[pg_test]
	fn test_sqids_encode() {
		let tests: HashMap<&str, Option<String>> = HashMap::from([
            ("SELECT sqids_encode(1, 2, 3)", Some("86Rf07".to_string())),
            ("SELECT sqids_encode(10::smallint, 1, 2, 3)", Some("86Rf07xd4z".to_string())),
            ("SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 1, 2, 3)", Some("XRKUdQ".to_string())),
            ("SELECT sqids_encode(array['86Rf07'], 1, 2, 3)", Some("se8ojk".to_string())),
            ("SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, 1, 2, 3)", Some("XRKUdQVBzg".to_string())),
            ("SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', array['XRKUdQ'], 1, 2, 3)", Some("WyXQfF".to_string())),
            ("SELECT sqids_encode(10::smallint, array['86Rf07'], 1, 2, 3)", Some("se8ojkCQvX".to_string())),
            ("SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, array['XRKUdQVBzg'], 1, 2, 3)", Some("WyXQfFQ21T".to_string())),
        ]);

		for (sql, expected) in tests {
			let result = Spi::get_one(sql).expect("Query failed");
			assert_eq!(result, expected, "Failed on query: {}", sql);
		}
	}

	#[pg_test]
	fn test_sqids_decode() {
		let tests: HashMap<&str, Option<Vec<i64>>> = HashMap::from([
            ("SELECT sqids_decode('86Rf07')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode(10::smallint, '86Rf07xd4z')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 'XRKUdQ')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode(array['86Rf07'], 'se8ojk')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, 'XRKUdQVBzg')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', array['XRKUdQ'], 'WyXQfF')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode(10::smallint, array['86Rf07'], 'se8ojkCQvX')", Some(vec![1, 2, 3])),
            ("SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, array['XRKUdQVBzg'], 'WyXQfFQ21T')", Some(vec![1, 2, 3])),
        ]);

		for (sql, expected) in tests {
			let result = Spi::get_one(sql).expect("Query failed");
			assert_eq!(result, expected, "Failed on query: {}", sql);
		}
	}
}

#[cfg(test)]
pub mod pg_test {
	pub fn setup(_options: Vec<&str>) {}

	pub fn postgresql_conf_options() -> Vec<&'static str> {
		vec![]
	}
}
