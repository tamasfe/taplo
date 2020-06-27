A "Rust library" file will be produced. This is used as an intermediate artifact and can be thought of as a
"static Rust library". These `rlib` files, unlike `staticlib` files, are
interpreted by the compiler in future linkage. This essentially means
that `rustc` will look for metadata in `rlib` files like it looks for metadata
in dynamic libraries. This form of output is used to produce statically linked
executables as well as `staticlib` outputs.