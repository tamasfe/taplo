The `lto` setting controls the [`-C lto` flag](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto) which controls LLVM's [link time optimizations](https://llvm.org/docs/LinkTimeOptimization.html). LTO can produce better optimized code, using
whole-program analysis, at the cost of longer linking time.
                    
See also the [`-C linker-plugin-lto`](https://doc.rust-lang.org/rustc/codegen-options/index.html#linker-plugin-lto) `rustc` flag for cross-language LTO.