A dynamic Rust library will be produced. This is different from the `lib` output type in that this forces
dynamic library generation. The resulting dynamic library can be used as a
dependency for other libraries and/or executables. This output type will
create `*.so` files on linux, `*.dylib` files on osx, and `*.dll` files on
windows.