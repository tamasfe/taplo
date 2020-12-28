use anyhow::anyhow;
use git2::{Delta, Repository, Sort, Tree};
use globset::Glob;
use hex::ToHex;
use path_clean::PathClean;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Write;
use std::{collections::HashSet, ffi::OsStr, path::PathBuf};
use structopt::StructOpt;
use tabwriter::TabWriter;
use taplo::schema::{SchemaExtraInfo, SchemaIndex, SchemaMeta};
use time::{Format, OffsetDateTime};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaStoreSchema {
    name: Option<String>,
    description: Option<String>,
    url: String,
    #[serde(default)]
    file_match: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaStoreCatalog {
    #[serde(default)]
    schemas: Vec<SchemaStoreSchema>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SchemaWithExtraInfo {
    title: Option<String>,
    description: Option<String>,
    #[serde(default, rename = "x-taplo-info")]
    extra: SchemaExtraInfo,
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Git repository
    #[structopt(long, default_value = ".")]
    git: PathBuf,

    /// Output JSON file
    #[structopt(short, long, default_value = "schema_index.json")]
    out: String,

    /// The base URL of the schemas.
    #[structopt(long, default_value = "https://taplo.tamasfe.dev/schemas")]
    url: String,

    /// Relative dir path from the Git repo directory.
    #[structopt(name = "DIR")]
    dir: PathBuf,

    /// Use schemastore.org for additional toml-compatible schemas.
    #[structopt(long)]
    schema_store: bool,
}

fn main() -> anyhow::Result<()> {
    let mut opt = Opt::from_args();

    opt.url = opt.url.trim_end_matches('/').into();

    let repo = Repository::discover(&opt.git)?;

    let mut revs = repo.revwalk().unwrap();
    revs.push_head().unwrap();
    revs.set_sorting(Sort::TIME).unwrap();

    let mut files = WalkDir::new(opt.git.join(&opt.dir))
        .into_iter()
        .filter_map(|res| {
            res.ok().map(|entry| entry.into_path()).and_then(|p| {
                if p.extension() == Some(OsStr::new("json")) {
                    Some(p.clean())
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>();

    let mut index = SchemaIndex::default();

    let mut new_tree: Option<Tree> = None;

    let mut tw = TabWriter::new(vec![]);

    for result in revs {
        let rev = match result {
            Ok(r) => r,
            Err(_) => continue,
        };

        if let Ok(commit) = repo.find_commit(rev) {
            let time = commit.time();

            let time_unix = time.seconds() + (time.offset_minutes() * 60) as i64;

            let old_tree = commit.tree().unwrap();

            let diff = repo
                .diff_tree_to_tree(Some(&old_tree), new_tree.as_ref(), None)
                .unwrap();

            let deltas = diff
                .deltas()
                .filter(|d| d.status() != Delta::Unmodified && d.status() != Delta::Deleted);

            for delta in deltas {
                let new_file = delta.new_file();

                if let Some(p) = new_file.path() {
                    if files.remove(p) {
                        let s: SchemaWithExtraInfo =
                            match serde_json::from_reader(std::fs::File::open(p).unwrap()) {
                                Ok(s) => s,
                                Err(err) => {
                                    panic!("invalid schema: {:?}: {}", p, err);
                                }
                            };

                        let name = p.file_name().unwrap().to_str().unwrap();

                        let url = format!("{}/{}", &opt.url, name);

                        let mut hasher = Sha256::new();
                        hasher.update(url.as_bytes());
                        let url_hash = hasher.finalize().encode_hex::<String>();

                        let updated =
                            OffsetDateTime::from_unix_timestamp(time_unix).format(Format::Rfc3339);

                        write!(&mut tw, "{}\t{:?}\t{}\n", name, delta.status(), &updated).unwrap();

                        index.schemas.push(SchemaMeta {
                            title: s.title,
                            description: s.description,
                            updated: Some(updated),
                            url,
                            url_hash,
                            extra: s.extra,
                        });
                    }
                }
            }

            if files.is_empty() {
                break;
            }

            new_tree = Some(old_tree);
        }
    }

    print!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());

    if !files.is_empty() {
        return Err(anyhow!("all files must be committed"));
    }

    if opt.schema_store {
        println!("Fetching Schema Store...");
        if let Err(err) = fetch_schema_store(&mut index) {
            println!("error fetching schema store: {}", err);
        }
    }

    serde_json::to_writer(std::fs::File::create(opt.out).unwrap(), &index)?;

    Ok(())
}

fn fetch_schema_store(index: &mut SchemaIndex) -> Result<(), anyhow::Error> {
    let catalog: SchemaStoreCatalog =
        reqwest::blocking::get("https://www.schemastore.org/api/json/catalog.json")?.json()?;

    let now_ts = OffsetDateTime::now_utc().format(Format::Rfc3339);
    let mut tw = TabWriter::new(vec![]);

    for schema in catalog.schemas {
        if !schema.file_match.iter().any(|m| m.ends_with(".toml")) {
            continue;
        }

        let mut hasher = Sha256::new();
        hasher.update(schema.url.as_bytes());
        let url_hash = hasher.finalize().encode_hex::<String>();

        let mut globs: Vec<Glob> = Vec::new();

        for fm in schema.file_match.iter().filter(|s| s.ends_with(".toml")) {
            match Glob::new(fm.trim_end_matches(".toml")) {
                Ok(glob) => {
                    globs.push(glob);
                }
                Err(_) => {
                    continue;
                }
            };
        }

        let sm = SchemaMeta {
            title: schema.name,
            description: schema.description,
            // We don't know.
            updated: Some(now_ts.clone()),
            url: schema.url,
            url_hash,
            extra: SchemaExtraInfo {
                authors: vec!["automatically included from https://schemastore.org".into()],
                patterns: globs
                    .into_iter()
                    .map(|g| {
                        let mut re = g.regex();

                        re = g
                            .regex()
                            .strip_suffix("$")
                            .unwrap_or(re)
                            .strip_prefix("(?-u)^")
                            .unwrap_or(re);

                        if g.regex().contains('*') {
                            format!(r#"{}\.toml$"#, re)
                        } else {
                            format!(r#"^(.*(/|\\){}\.toml|{}\.toml)$"#, re, re)
                        }
                    })
                    .collect(),
                ..Default::default()
            },
        };

        write!(
            &mut tw,
            "{}\t{}\n",
            sm.title.clone().unwrap_or_default(),
            sm.url
        )
        .unwrap();

        index.schemas.push(sm);
    }

    println!("\nNew Schema Store schemas:");
    print!("{}", String::from_utf8(tw.into_inner().unwrap()).unwrap());

    Ok(())
}
