/*
Types based on Cargo's definitions.

They're modified for better schema generation:

- "Toml" prefixes are removed from the names.
- Some external types were replaced with local ones.
- Some types were renamed to be more specialized and user-friendly.

This module has to be updated if the Cargo definitions change.
*/
#![allow(dead_code)]

use schemars::JsonSchema;
use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf};

type LibTarget = Target;
type BinTarget = Target;
type ExampleTarget = Target;
type TestTarget = Target;
type BenchTarget = Target;

#[derive(Clone, Debug, Serialize, JsonSchema)]
#[serde(untagged)]
#[schemars(title = "Dependency")]
pub enum Dependency {
    Simple(String),
    Detailed(DetailedDependency),
}

#[derive(Serialize, JsonSchema, Clone, Debug, Default)]
#[serde(rename_all = "kebab-case")]
#[schemars(title = "Detailed Dependency")]
pub struct DetailedDependency {
    version: Option<String>,
    registry: Option<String>,
    /// The URL of the `registry` field.
    /// This is an internal implementation detail. When Cargo creates a
    /// package, it replaces `registry` with `registry-index` so that the
    /// manifest contains the correct URL. All users won't have the same
    /// registry names configured, so Cargo can't rely on just the name for
    /// crates published by other users.
    registry_index: Option<String>,
    path: Option<String>,
    git: Option<String>,
    branch: Option<String>,
    tag: Option<String>,
    rev: Option<String>,
    features: Option<Vec<String>>,
    optional: Option<bool>,
    default_features: Option<bool>,
    #[serde(rename = "default_features")]
    default_features2: Option<bool>,
    package: Option<String>,
    public: Option<bool>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
#[schemars(title = "Cargo")]
#[serde(deny_unknown_fields)]
pub struct Manifest {
    cargo_features: Option<Vec<String>>,
    package: Option<Box<Project>>,
    project: Option<Box<Project>>,
    profile: Option<Profiles>,
    lib: Option<LibTarget>,
    bin: Option<Vec<BinTarget>>,
    example: Option<Vec<ExampleTarget>>,
    test: Option<Vec<TestTarget>>,
    bench: Option<Vec<TestTarget>>,
    dependencies: Option<BTreeMap<String, Dependency>>,
    dev_dependencies: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "dev_dependencies")]
    dev_dependencies2: Option<BTreeMap<String, Dependency>>,
    build_dependencies: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "build_dependencies")]
    build_dependencies2: Option<BTreeMap<String, Dependency>>,
    features: Option<BTreeMap<String, Vec<String>>>,
    target: Option<BTreeMap<String, Platform>>,
    replace: Option<BTreeMap<String, Dependency>>,
    patch: Option<BTreeMap<String, BTreeMap<String, Dependency>>>,
    workspace: Option<Workspace>,
    badges: Option<BTreeMap<String, BTreeMap<String, String>>>,
}

#[derive(Serialize, JsonSchema, Clone, Debug, Default)]
#[schemars(title = "Profiles")]
pub struct Profiles(BTreeMap<String, Profile>);

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
#[schemars(title = "Optimization Level")]
pub enum OptLevel {
    // enum of {1,2,3,"s","z"}
    Int(i32),
    String(String),
}

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
#[schemars(title = "Debug Level")]
// enum of 0, 1, 2
pub enum DebugLevel {
    U32(u32),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(transparent)]
#[schemars(title = "Panic")]
// enum of "unwind" or "abort"
pub struct Panic(String);

#[derive(Serialize, JsonSchema, Clone, Debug, Default, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
#[schemars(title = "Profile")]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub opt_level: Option<OptLevel>,
    pub lto: Option<Lto>,
    pub codegen_units: Option<u32>,
    pub debug: Option<DebugLevel>,
    pub debug_assertions: Option<bool>,
    pub rpath: Option<bool>,
    pub panic: Option<Panic>,
    pub overflow_checks: Option<bool>,
    pub incremental: Option<bool>,

    pub package: Option<BTreeMap<PackageSpec, Profile>>,
    pub build_override: Option<Box<Profile>>,
    pub dir_name: Option<String>,
    pub inherits: Option<String>,
}

/*
Specs like:
- "https://crates.io/foo#1.2.3",
- "https://crates.io/foo#bar:1.2.3",
- "crates.io/foo",
- "crates.io/foo#1.2.3",
- "crates.io/foo#bar",
- "crates.io/foo#bar:1.2.3",
- "foo",
- "foo:1.2.3",
- "*"
*/
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Debug, Serialize, JsonSchema)]
#[schemars(title = "Package Spec")]
pub struct PackageSpec(String);

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[schemars(title = "Meta Build")]
pub struct MetaBuild(Vec<String>);

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
pub enum StringOrBool {
    String(String),
    Bool(bool),
}

#[derive(Clone, Debug, Serialize, JsonSchema, Eq, PartialEq)]
#[serde(untagged)]
#[schemars(title = "Lto")]
pub enum Lto {
    // enum of "fat", "thin", "off"
    String(String),
    Bool(bool),
}

#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(untagged)]
#[schemars(title = "Publish")]
pub enum Publish {
    VecString(Vec<String>),
    Disable(Disable)
}

#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(transparent)]
#[schemars(title = "Semantic Version")]
// regex ^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$
pub struct SemVer(String);

#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(untagged)]
#[schemars(title = "Build")]
pub enum Build {
    String(String),
    Disable(Disable)
}

#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(transparent)]
#[schemars(title = "Disable")]
// Enum of only false
pub struct Disable(bool);

/// Represents the `package`/`project` sections of a `Cargo.toml`.
///
/// Note that the order of the fields matters, since this is the order they
/// are serialized to a TOML file. For example, you cannot have values after
/// the field `metadata`, since it is a table and values cannot appear after
/// tables.
#[derive(Serialize, JsonSchema, Clone, Debug)]
#[serde(deny_unknown_fields)]
#[schemars(title = "Project")]
pub struct Project {
    edition: Option<String>,
    name: String,
    version: SemVer,
    authors: Option<Vec<String>>,
    build: Option<Build>,
    metabuild: Option<MetaBuild>,
    links: Option<String>,
    exclude: Option<Vec<String>>,
    include: Option<Vec<String>>,
    publish: Option<Publish>,
    #[serde(rename = "publish-lockfile")]
    publish_lockfile: Option<bool>,
    workspace: Option<String>,
    #[serde(rename = "im-a-teapot")]
    im_a_teapot: Option<bool>,
    autobins: Option<bool>,
    autoexamples: Option<bool>,
    autotests: Option<bool>,
    autobenches: Option<bool>,
    #[serde(rename = "namespaced-features")]
    namespaced_features: Option<bool>,
    #[serde(rename = "default-run")]
    default_run: Option<String>,

    // Package metadata.
    description: Option<String>,
    homepage: Option<String>,
    documentation: Option<String>,
    readme: Option<String>,
    keywords: Option<Vec<String>>,
    categories: Option<Vec<String>>,
    license: Option<String>,
    #[serde(rename = "license-file")]
    license_file: Option<String>,
    repository: Option<String>,
    metadata: Option<BTreeMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, JsonSchema)]
#[schemars(title = "Workspace")]
#[serde(deny_unknown_fields)]
pub struct Workspace {
    members: Option<Vec<String>>,
    #[serde(rename = "default-members")]
    default_members: Option<Vec<String>>,
    exclude: Option<Vec<String>>,
}

#[derive(Default, Serialize, JsonSchema, Debug, Clone)]
#[schemars(title = "Target")]
#[serde(deny_unknown_fields)]
struct Target {
    name: Option<String>,

    // The intention was to only accept `crate-type` here but historical
    // versions of Cargo also accepted `crate_type`, so look for both.
    #[serde(rename = "crate-type")]
    crate_type: Option<Vec<String>>,
    #[serde(rename = "crate_type")]
    crate_type2: Option<Vec<String>>,

    path: Option<PathValue>,
    test: Option<bool>,
    doctest: Option<bool>,
    bench: Option<bool>,
    doc: Option<bool>,
    plugin: Option<bool>,
    #[serde(rename = "proc-macro")]
    proc_macro: Option<bool>,
    #[serde(rename = "proc_macro")]
    proc_macro2: Option<bool>,
    harness: Option<bool>,
    #[serde(rename = "required-features")]
    required_features: Option<Vec<String>>,
    edition: Option<String>,
}

type PathValue = PathBuf;

/// Corresponds to a `target` entry, but `Target` is already used.
#[derive(Serialize, JsonSchema, Debug)]
#[schemars(title = "Platform")]
#[serde(deny_unknown_fields)]
struct Platform {
    dependencies: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "build-dependencies")]
    build_dependencies: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "build_dependencies")]
    build_dependencies2: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "dev-dependencies")]
    dev_dependencies: Option<BTreeMap<String, Dependency>>,
    #[serde(rename = "dev_dependencies")]
    dev_dependencies2: Option<BTreeMap<String, Dependency>>,
}
