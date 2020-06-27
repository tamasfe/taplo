The `panic` setting controls the [`-C panic` flag](https://doc.rust-lang.org/rustc/codegen-options/index.html#panic) which controls which panic
strategy to use.

When set to `"unwind"`, the actual value depends on the default of the target
platform. For example, the NVPTX platform does not support unwinding, so it
always uses `"abort"`.

Tests, benchmarks, build scripts, and proc macros ignore the `panic` setting.
The `rustc` test harness currently requires `unwind` behavior. See the
[`panic-abort-tests`](https://doc.rust-lang.org/cargo/reference/unstable.html#panic-abort-tests) unstable flag which enables `abort` behavior.

Additionally, when using the `abort` strategy and building a test, all of the
dependencies will also be forced to built with the `unwind` strategy.