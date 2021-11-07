## ZeroToProduction Rust

<br/>

### Integration Tests

`tests` folder contains the integration tests. Consider these as the "black box testing" approach. This means that during testing, interactions with the API are similar with the ones that are in the real world / deployment. The calls are getting into the system the same way as would otherwise be done by external clients.

#### Test Log Output

Note that if you want to log something in the tests, to see the output you need to run the tests using `cargo test -- --nocapture`. By default (or otherwise), you'll get it only in case of test failure.

This works well with `print(ln)` approach. If you want to use `log!` macro, see details [here](https://github.com/rust-lang/log/issues/106).

<br/>
