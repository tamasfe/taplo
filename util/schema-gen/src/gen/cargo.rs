/**
This is low-effort copy-paste on purpose, I don't expect
to run this more often than every few months.
*/
use schemars::schema::SchemaObject;
use serde_json::json;

use super::KeyVisitor;
use taplo_ide::schema::{ExtDocs, ExtLinks, ExtMeta, EXTENSION_KEY};

#[derive(Debug, Clone)]
pub struct CargoVisitor;

impl schemars::visit::Visitor for CargoVisitor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        if let Some(title) = schema.metadata.as_ref().and_then(|meta| meta.title.clone()) {
            match title.as_str() {
                "Publish" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-publish-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/publish.md").into())
                }
                "Semantic Version" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some("https://doc.rust-lang.org/cargo/reference/manifest.html#the-version-field".into()),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().default = Some(json!("0.1.0"));
                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/semver.md").into());

                    // SemVer regex: https://semver.org/#is-there-a-suggested-regular-expression-regex-to-check-a-semver-string
                    schema.string().pattern = Some(
                        r#"^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$"#.into(),
                    );
                }
                "Semantic Version Requirement" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some("https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html".into()),
                        ..Default::default()
                    });

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/semver_req.md").into());

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().default = Some(json!("*"));
                    // Regex that combines the SemVer regex above, or a list of Cargo version requirements.
                    schema.string().pattern = Some(
                    r#"^((((>=|>|<|=|\^|~)?\s*[0-9]+(.[0-9]+)?(.[0-9]+)?|([0-9]+|\*)(.([0-9]+|\*))?(.([0-9]+|\*))?)+)(,\s*(((>=|>|<|=|\^|~)?\s*[0-9]+(.[0-9]+)?(.[0-9]+)?|([0-9]+|\*)(.([0-9]+|\*))?(.([0-9]+|\*))?)))*|(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?)$"#.into()
                );
                }
                "Build" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-build-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/build.md").into())
                }
                "Edition" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/stable/edition-guide/introduction.html"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/edition.md").into());

                    schema.enum_values = Some(vec![json!("2015"), json!("2018")]);
                }
                "Profiles" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some("https://doc.rust-lang.org/cargo/reference/profiles.html".into()),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile.md").into())
                }
                "Lto" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#lto".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        enum_values: Some(vec![
                            Some(
                                include_str!("../../descriptions/cargo/profile/lto_fat.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/lto_thin.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/lto_off.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/lto_fat.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/lto_false.md")
                                    .into(),
                            ),
                        ]),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/lto.md").into());

                    schema.enum_values = Some(vec![
                        json!("fat"),
                        json!("thin"),
                        json!("off"),
                        json![true],
                        json![false],
                    ]);
                }
                "Optimization Level" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#opt-level"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        enum_values: Some(vec![
                            Some(include_str!("../../descriptions/cargo/profile/opt_0.md").into()),
                            Some(include_str!("../../descriptions/cargo/profile/opt_1.md").into()),
                            Some(include_str!("../../descriptions/cargo/profile/opt_2.md").into()),
                            Some(include_str!("../../descriptions/cargo/profile/opt_3.md").into()),
                            Some(include_str!("../../descriptions/cargo/profile/opt_s.md").into()),
                            Some(include_str!("../../descriptions/cargo/profile/opt_z.md").into()),
                        ]),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/opt.md").into());

                    schema.enum_values = Some(vec![
                        json!(0),
                        json!(1),
                        json!(2),
                        json!(3),
                        json!["s"],
                        json!["z"],
                    ]);
                }
                "Debug Level" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#debug".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        enum_values: Some(vec![
                            Some(
                                include_str!("../../descriptions/cargo/profile/debug_0.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/debug_1.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/debug_2.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/debug_2.md").into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/debug_0.md").into(),
                            ),
                        ]),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/debug.md").into());

                    schema.enum_values = Some(vec![
                        json!(0),
                        json!(1),
                        json!(2),
                        json![true],
                        json![false],
                    ]);
                }
                "Package" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-package-section".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/package.md").into());
                }
                "Panic" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#panic".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        enum_values: Some(vec![
                            Some(
                                include_str!("../../descriptions/cargo/profile/panic_unwind.md")
                                    .into(),
                            ),
                            Some(
                                include_str!("../../descriptions/cargo/profile/panic_abort.md")
                                    .into(),
                            ),
                        ]),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/panic.md").into());

                    schema.enum_values = Some(vec![json!("unwind"), json!("abort")]);
                }
                "Readme" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/readme.md").into())
                }
                "Workspace" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/workspaces.html".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/workspace/workspace.md").into())
                }
                _ => {}
            };
        }

        if let Some(keys) = schema
            .extensions
            .get(KeyVisitor::KEYS_NAME)
            .map(|k| serde_json::from_value::<Vec<String>>(k.clone()).unwrap())
        {
            let dotted = keys.join(".");

            match dotted.as_str() {
                "Profile.debug-assertions" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#debug-assertions".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/profile/debug-assertions.md").into(),
                    )
                }
                "Profile.build-override" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#overrides"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(
                            include_str!("../../descriptions/cargo/profile/build-override.md")
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Profile.overflow-checks" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#overflow-checks".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/profile/overflow-checks.md").into(),
                    )
                }
                "Profile.package" => {
                    // TODO regex for package id spec
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#overrides"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/package.md").into())
                }
                "Profile.incremental" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#incremental"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/incremental.md").into())
                }
                "Profile.codegen-units" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#codegen-units"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/profile/codegen-units.md").into(),
                    )
                }
                "Profile.rpath" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#rpath".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/profile/rpath.md").into())
                }
                "Package.name" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/name.md").into())
                }
                "Package.authors" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-authors-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/authors.md").into())
                }
                "Package.autobins" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/autobins.md").into())
                }
                "Package.autotests" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/autotests.md").into())
                }
                "Package.autobenches" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/autobenches.md").into())
                }
                "Package.autoexamples" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#target-auto-discovery".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/package/autoexamples.md").into(),
                    )
                }
                "Package.description" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-description-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/description.md").into())
                }
                "Package.documentation" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-documentation-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/package/documentation.md").into(),
                    )
                }
                "Package.readme" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-readme-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/readme.md").into())
                }
                "Package.homepage" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-homepage-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/homepage.md").into())
                }
                "Package.repository" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-repository-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/repository.md").into())
                }
                "Package.license" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/license.md").into())
                }
                "Package.license-file" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-license-and-license-file-fields".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/package/license-file.md").into(),
                    )
                }
                "Package.keywords" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-keywords-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/keywords.md").into())
                }
                "Package.categories" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-categories-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/categories.md").into())
                }
                "Package.workspace" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-workspace-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/workspace.md").into())
                }
                "Package.links" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-links-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/links.md").into())
                }
                "Package.exclude" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/package/exclude_include.md").into(),
                    )
                }
                "Package.include" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-exclude-and-include-fields".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/package/exclude_include.md").into(),
                    )
                }

                "Package.metadata" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-metadata-table".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/metadata.md").into())
                }
                "Package.default-run" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-default-run-field".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/default-run.md").into())
                }
                "Package.im-a-teapot" => {
                    let mut ext = ExtMeta::default();
                    ext.hidden = Some(true);

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/package/im-a-teapot.md").into())
                }
                "Cargo.badges" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/badges.md").into())
                }
                "Profiles.dev" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#dev".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(include_str!("../../descriptions/cargo/profiles_dev.md").into()),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Profiles.release" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#release"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(
                            include_str!("../../descriptions/cargo/profiles_release.md").into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Profiles.test" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#test".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(
                            include_str!("../../descriptions/cargo/profiles_test.md").into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Profiles.bench" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/profiles.html#bench".into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(
                            include_str!("../../descriptions/cargo/profiles_bench.md").into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Cargo.features" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some("https://doc.rust-lang.org/cargo/reference/features.html".into()),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/features.md").into())
                }
                "Cargo.patch" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/overriding-dependencies.html#the-patch-section".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/patch.md").into())
                }
                "Cargo.dependencies" | "Platform.dependencies" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependencies.md").into())
                }
                "Cargo.dev-dependencies" | "Platform.dev-dependencies" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#development-dependencies".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dev-dependencies.md").into())
                }
                "Cargo.build-dependencies" | "Platform.build-dependencies" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#build-dependencies".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/build-dependencies.md").into())
                }
                "Cargo.lib" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#library"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        main: Some(
                            include_str!("../../descriptions/cargo/cargo_target/target_lib.md")
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Cargo.bin" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#binaries"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/target_bin.md").into(),
                    )
                }
                "Cargo.test" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/target_test.md").into(),
                    )
                }
                "Cargo.example" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#examples"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/target_example.md")
                            .into(),
                    )
                }
                "Cargo.bench" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#benchmarks".into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/target_bench.md")
                            .into(),
                    )
                }
                "Workspace.members" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/workspace/members.md").into())
                }
                "Workspace.exclude" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/workspace/exclude.md").into())
                }
                "Workspace.default-members" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/workspace/default-members.md")
                            .into(),
                    )
                }
                "Workspace.metadata" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/workspaces.html#the-workspace-section"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/workspace/metadata.md").into())
                }
                "Target.name" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-name-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/cargo_target/name.md").into())
                }
                "Target.path" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-path-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/cargo_target/path.md").into())
                }
                "Target.doctest" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-doctest-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/doctest.md").into(),
                    )
                }
                "Target.bench" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-bench-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/cargo_target/bench.md").into())
                }
                "Target.doc" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-doc-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/cargo_target/doc.md").into())
                }
                "Target.proc-macro" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-proc-macro-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/proc-macro.md").into(),
                    )
                }
                "Target.test" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-test-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/cargo_target/test.md").into())
                }
                "Target.harness" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-harness-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/harness.md").into(),
                    )
                }
                "Target.crate-type" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-crate-type-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    ext.docs = Some(ExtDocs {
                        enum_values: Some(vec![
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_bin.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_lib.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_rlib.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_dylib.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_cdylib.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_staticlib.md"
                                )
                                .into(),
                            ),
                            Some(
                                include_str!(
                                    "../../descriptions/cargo/cargo_target/crate-type_proc-macro.md"
                                )
                                .into(),
                            ),
                        ]),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/crate-type.md").into(),
                    )
                }
                "Target.required-features" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/cargo-targets.html#the-required-features-field"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/cargo_target/required-features.md")
                            .into(),
                    )
                }
                "Detailed Dependency" => {
                    let mut ext = ExtMeta::default();

                    ext.init_fields = Some(
                        vec!["version".into()]
                    );

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                "Detailed Dependency.branch" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/branch.md").into())
                }
                "Detailed Dependency.git" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/git.md").into())
                }
                "Detailed Dependency.tag" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/tag.md").into())
                }
                "Detailed Dependency.rev" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/rev.md").into())
                }
                "Detailed Dependency.path" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-path-dependencies"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/path.md").into())
                }
                "Detailed Dependency.features" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/features.md").into())
                }
                "Detailed Dependency.default-features" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description = Some(
                        include_str!("../../descriptions/cargo/dependency/default-features.md")
                            .into(),
                    )
                }
                "Detailed Dependency.optional" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/optional.md").into())
                }
                "Detailed Dependency.registry" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-other-registries"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/registry.md").into())
                }
                "Detailed Dependency.package" => {
                    let mut ext = ExtMeta::default();

                    ext.links = Some(ExtLinks {
                        key: Some(
                            "https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#renaming-dependencies-in-cargotoml"
                                .into(),
                        ),
                        ..Default::default()
                    });

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());

                    schema.metadata().description =
                        Some(include_str!("../../descriptions/cargo/dependency/package.md").into())
                }
                "Cargo.replace"
                | "Profile.dir-name"
                | "Profile.inherits"
                | "Cargo.dev_dependencies"
                | "Package.namespaced-features"
                | "Package.publish-lockfile"
                | "Package.metabuild"
                | "Platform.dev_dependencies"
                | "Cargo.project"
                | "Target.crate_type"
                | "Target.proc_macro"
                | "Target.plugin"
                | "Platform.build_dependencies"
                | "Cargo.build_dependencies"
                | "Detailed Dependency.default_features"
                | "Detailed Dependency.public"
                | "Detailed Dependency.registry-index" => {
                    let mut ext = ExtMeta::default();
                    ext.hidden = Some(true);

                    schema
                        .extensions
                        .insert(EXTENSION_KEY.into(), serde_json::to_value(ext).unwrap());
                }
                _ => {}
            }
        }

        schemars::visit::visit_schema_object(self, schema)
    }
}
