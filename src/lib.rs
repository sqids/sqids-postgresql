use pgrx::prelude::*;
use sqids::Sqids;

use error::PgError;

mod error;
mod utils;

pgrx::pg_module_magic!();

#[pg_extern(name = "sqids_encode")]
fn sqids_encode(numbers: VariadicArray<i64>) -> Result<String, PgError> {
    let sqids = Sqids::builder().build().map_err(PgError::SqidsError)?;

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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

    sqids
        .encode(&utils::process_numbers(numbers)?)
        .map_err(PgError::SqidsError)
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
