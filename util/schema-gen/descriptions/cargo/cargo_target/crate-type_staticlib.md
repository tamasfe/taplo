A static system library will be produced. This is different from other library outputs in that
the compiler will never attempt to link to `staticlib` outputs. The
purpose of this output type is to create a static library containing all of
the local crate's code along with all upstream dependencies. The static
library is actually a `*.a` archive on linux and osx and a `*.lib` file on
windows. This format is recommended for use in situations such as linking
Rust code into an existing non-Rust application because it will not have
dynamic dependencies on other Rust code.