## ZeroToProduction Rust

<br/>

### Prereqs

The following software needs to be installed locally:

- PostgreSQL Client
  - On Ubuntu based distros, use `sudo apt search postgresql-client-13`
- `sqlx-cli`
  - Install it using `cargo install --version=0.5.9 sqlx-cli --no-default-features --features postgres`

Note that `.env` file contains a DATABASE_URL variable that is used by `sqlx::query!` macros for performing compile time checks to validate the queries. Therefore, if you change something in `scripts/init_db.sh` script, make sure to reflect those changes in this file as well.

<br/>

### Run

Before starting up the service, Postgres db access and provisioning must be done.<br/>
Run `./scripts/init_db.sh` script that starts a PostgreSQL instance as a Docker container and runs the database migrations within.
If the container is already running, you can skip the container bootstrap using `SKIP_DOCKER=true ./scripts/init_db.sh`.

Start the service using `run.sh` (which it's just a convenince and minimal script for doing `cargo run`,
enough needed since there is one single binary `[[bin]]` entry into `Cargo.toml` file).

<br/>

### Integration Tests

`tests` folder contains the integration tests. Consider these as the "black box testing" approach. This means that during testing, interactions with the API are similar with the ones that are in the real world / deployment. The calls are getting into the system the same way as would otherwise be done by external clients.

#### Test Log Output

Note that if you want to log something in the tests, to see the output you need to run the tests using `cargo test -- --nocapture`. By default (or otherwise), you'll get it only in case of test failure.

This works well with `print` and `println!` approach. If you want to use `log!` macro, see details [here](https://github.com/rust-lang/log/issues/106).

<br/>
