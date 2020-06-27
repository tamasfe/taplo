The output produced is not specified, but if a `-L` path is provided to it then the
compiler will recognize the output artifacts as a macro and it can be loaded
for a program. Crates compiled with this crate type  must only export
[procedural macros](https://doc.rust-lang.org/reference/procedural-macros.html). The compiler will automatically set the `proc_macro`
[configuration option](https://doc.rust-lang.org/reference/conditional-compilation.html). The crates are always compiled with the same target
that the compiler itself was built with. For example, if you are executing
the compiler from Linux with an `x86_64` CPU, the target will be
`x86_64-unknown-linux-gnu` even if the crate is a dependency of another crate
being built for a different target.