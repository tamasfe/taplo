Cargo bakes in the concept of [Semantic Versioning](https://semver.org/), so make sure you follow some basic rules:

* Before you reach 1.0.0, anything goes, but if you make breaking changes,
    increment the minor version. In Rust, breaking changes include adding fields to
    structs or variants to enums.
* After 1.0.0, only make breaking changes when you increment the major version.
    Don’t break the build.
* After 1.0.0, don’t add any new public API (no new `pub` anything) in patch-level
    versions. Always increment the minor version if you add any new `pub` structs,
    traits, fields, types, functions, methods or anything else.
* Use version numbers with three numeric parts such as 1.0.0 rather than 1.0.