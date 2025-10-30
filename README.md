# rust_command_line
A repo for Command Line Rust

## Setup Project
### Create a Repo in GitHub.com
* `git clone`

### Init a Cargo Project
* `cargo init`

## `echo`

## `cat`

## Test

### Run a single Rust test by name
You can run one test function (like `dies_bad_bytes`) using `cargo test` with its name. Because this test lives in an integration test file `tests/cli.rs`, the test binary will be named `cli`.

#### From the crate’s directory (recommended)
- Run only that test across all test binaries (good enough in many cases):
  ```bash
  cargo test dies_bad_bytes
  ```
- Run only from the `cli` integration-test binary (more precise):
  ```bash
  cargo test --test cli dies_bad_bytes
  ```
- Exact match (avoid substring matches):
  ```bash
  cargo test --test cli dies_bad_bytes -- --exact
  ```
- See test output (don’t capture stdout/stderr):
  ```bash
  cargo test --test cli dies_bad_bytes -- --exact --nocapture
  ```

#### From a workspace root
If your project is a workspace, specify the package (replace `PACKAGE_NAME` with the `[package].name` from that crate’s `Cargo.toml`):
- Run the single test in the `cli` integration-test binary:
  ```bash
  cargo test -p PACKAGE_NAME --test cli dies_bad_bytes
  ```
- With exact match and no capture:
  ```bash
  cargo test -p PACKAGE_NAME --test cli dies_bad_bytes -- --exact --nocapture
  ```

#### Tips
- List available tests to confirm names:
  ```bash
  cargo test -- --list
  ```
  Or just for the `cli` test binary:
  ```bash
  cargo test --test cli -- --list
  ```
- The test you showed is in `ch4/tests/cli.rs`, so if you’re at the repo root, you can `cd ch4` first, then use the commands above.

That’s it—pick the variant that fits your setup and it will run only `dies_bad_bytes`. 

