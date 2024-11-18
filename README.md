[![Benchmark](https://github.com/RGGH/criterion_bench/actions/workflows/benchmark.yml/badge.svg)](https://github.com/RGGH/criterion_bench/actions/workflows/benchmark.yml) [![Rust](https://github.com/RGGH/criterion_bench/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/criterion_bench/actions/workflows/rust.yml)

[The Book - bench](https://doc.rust-lang.org/cargo/commands/cargo-bench.html)

[benchmarking rust code](https://engineering.deptagency.com/benchmarking-rust-code-using-criterion-rs)

### Add to Cargo.toml 
  
  ```
  [dev-dependencies]
  criterion = {version = "0.3", features = ["html_reports"]}

  [[bench]]
  name = "euler1_benchmark"
  harness = false
