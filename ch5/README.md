# Ch5 wcr

## Run test from main.rs
- Limit to the library target explicitly (useful if the crate also has binaries/integration tests):
  ```bash
  cargo test -p ch5 --lib test_count
  ```

- Using the module path filter (sometimes helpful in larger modules):
  ```bash
  cargo test -p ch5 tests::test_count -- --exact
  ```

## Test on Linux