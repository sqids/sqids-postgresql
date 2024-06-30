# [Sqids PostgreSQL](https://sqids.org/postgresql)

[![Github Actions](https://img.shields.io/github/actions/workflow/status/sqids/sqids-postgresql/tests.yml)](https://github.com/sqids/sqids-postgresql/actions)

[Sqids](https://sqids.org/postgresql) (*pronounced "squids"*) is a small library that lets you **generate unique IDs from numbers**. It's good for link shortening, fast & URL-safe ID generation and decoding back into numbers for quicker database lookups.

Features:

- **Encode multiple numbers** - generate short IDs from one or several non-negative numbers
- **Quick decoding** - easily decode IDs back into numbers
- **Unique IDs** - generate unique IDs by shuffling the alphabet once
- **ID padding** - provide minimum length to make IDs more uniform
- **URL safe** - auto-generated IDs do not contain common profanity
- **Randomized output** - Sequential input provides nonconsecutive IDs
- **Many implementations** - Support for [40+ programming languages](https://sqids.org/)

## 🧰 Use-cases

Good for:

- Generating IDs for public URLs (eg: link shortening)
- Generating IDs for internal systems (eg: event tracking)
- Decoding for quicker database lookups (eg: by primary keys)

Not good for:

- Sensitive data (this is not an encryption library)
- User IDs (can be decoded revealing user count)

## 🚀 Getting started

### Debugging

1. [Install Rust](https://www.rust-lang.org/) if you don't have it.

1. Install [pgrx](https://github.com/pgcentralfoundation/pgrx?tab=readme-ov-file#getting-started):

    ```bash
    cargo install --locked cargo-pgrx
    ```

1. Install dependencies and run psql session:

    ```bash
    cargo build
    cargo pgrx run
    ```

1. Install extension:

    ```sql
    DROP EXTENSION pg_sqids;
    CREATE EXTENSION pg_sqids;
    ```

1. Run sample query:

    ```sql
    SELECT sqids_encode(1, 2, 3); -- 86Rf07
    ```

### Installing

1. Create the extension:

    ```bash
    cargo pgrx package
    ```

1. Extension files should be in `target/release`

1. Install the extension:

    ```sql
    DROP EXTENSION pg_sqids;
    CREATE EXTENSION pg_sqids;
    ```

## 👩‍💻 Examples

Simple encode & decode:

```sql
SELECT sqids_encode(1, 2, 3); -- 86Rf07
SELECT sqids_decode('86Rf07'); -- {1,2,3}
```

> **Note**
> 🚧 Because of the algorithm's design, **multiple IDs can decode back into the same sequence of numbers**. If it's important to your design that IDs are canonical, you have to manually re-encode decoded numbers and check that the generated ID matches.

Enforce a *minimum* length for IDs:

```sql
SELECT sqids_encode(10::smallint, 1, 2, 3); -- 86Rf07xd4z
SELECT sqids_decode(10::smallint, '86Rf07xd4z'); -- {1,2,3}
```

Randomize IDs by providing a custom alphabet:

```sql
SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 1, 2, 3); -- XRKUdQ
SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 'XRKUdQ'); -- {1,2,3}
```

Prevent specific words from appearing anywhere in the auto-generated IDs:

```sql
SELECT sqids_encode(array['86Rf07'], 1, 2, 3); -- se8ojk
SELECT sqids_decode(array['86Rf07'], 'se8ojk'); -- {1,2,3}
```

### Using multiple parameters

Alphabet + min length:

```sql
SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, 1, 2, 3); -- XRKUdQVBzg
SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, 'XRKUdQVBzg'); -- {1,2,3}
```

Alphabet + blocklist:

```sql
SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', array['XRKUdQ'], 1, 2, 3); -- WyXQfF
SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', array['XRKUdQ'], 'WyXQfF'); -- {1,2,3}
```

Min length + blocklist:

```sql
SELECT sqids_encode(10::smallint, array['86Rf07'], 1, 2, 3); -- se8ojkCQvX
SELECT sqids_decode(10::smallint, array['86Rf07'], 'se8ojkCQvX'); -- {1,2,3}
```

Alphabet + min length + blocklist:

```sql
SELECT sqids_encode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, array['XRKUdQVBzg'], 1, 2, 3); -- WyXQfFQ21T
SELECT sqids_decode('k3G7QAe51FCsPW92uEOyq4Bg6Sp8YzVTmnU0liwDdHXLajZrfxNhobJIRcMvKt', 10::smallint, array['XRKUdQVBzg'], 'WyXQfFQ21T'); -- {1,2,3}
```

## 📝 License

[MIT](LICENSE)
