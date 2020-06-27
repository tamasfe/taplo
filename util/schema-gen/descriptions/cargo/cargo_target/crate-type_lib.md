A Rust library will be produced.
This is an ambiguous concept as to what exactly is produced because a library
can manifest itself in several forms. The purpose of this generic `lib` option
is to generate the "compiler recommended" style of library. The output library
will always be usable by rustc, but the actual type of library may change from
time-to-time. The remaining output types are all different flavors of
libraries, and the `lib` type can be seen as an alias for one of them (but the
actual one is compiler-defined).