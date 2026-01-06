use globset::{Glob, GlobSetBuilder};
use percent_encoding::percent_decode_str;
use serde_json::Value;
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
    path::{Path, PathBuf},
    sync::Arc,
};

#[derive(Debug, Clone)]
pub struct GlobRule {
    include: globset::GlobSet,
    exclude: globset::GlobSet,
}

impl GlobRule {
    pub fn new(
        include: impl IntoIterator<Item = impl AsRef<str>>,
        exclude: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<Self, anyhow::Error> {
        let mut inc = GlobSetBuilder::new();
        for glob in include {
            inc.add(Glob::new(glob.as_ref())?);
        }

        let mut exc = GlobSetBuilder::new();
        for glob in exclude {
            exc.add(Glob::new(glob.as_ref())?);
        }

        Ok(Self {
            include: inc.build()?,
            exclude: exc.build()?,
        })
    }

    pub fn is_match(&self, text: impl AsRef<Path>) -> bool {
        if !self.include.is_match(text.as_ref()) {
            return false;
        }

        !self.exclude.is_match(text.as_ref())
    }
}

#[derive(Eq)]
pub struct ArcHashValue(pub Arc<Value>);

impl Hash for ArcHashValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        HashValue(&self.0).hash(state);
    }
}

impl PartialEq for ArcHashValue {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[derive(Eq)]
pub struct HashValue<'v>(pub &'v Value);

impl PartialEq for HashValue<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Hash for HashValue<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match &self.0 {
            Value::Null => 0.hash(state),
            Value::Bool(v) => v.hash(state),
            Value::Number(v) => v.hash(state),
            Value::String(v) => v.hash(state),
            Value::Array(v) => {
                for v in v {
                    HashValue(v).hash(state);
                }
            }
            Value::Object(v) => {
                for (k, v) in v {
                    k.hash(state);
                    HashValue(v).hash(state);
                }
            }
        }
    }
}

pub trait Normalize {
    /// Normalizing in the context of Taplo the following:
    ///
    /// - replaces `\` with `/` on windows
    /// - decodes all percent-encoded characters
    #[must_use]
    fn normalize(self) -> Self;
}

impl Normalize for PathBuf {
    fn normalize(self) -> Self {
        match self.to_str() {
            Some(s) => (*normalize_str(s)).into(),
            None => self,
        }
    }
}

pub(crate) fn normalize_str(s: &str) -> Cow<'_, str> {
    let Some(percent_decoded) = percent_decode_str(s).decode_utf8().ok() else {
        return s.into();
    };

    if cfg!(windows) {
        percent_decoded.replace('\\', "/").into()
    } else {
        percent_decoded
    }
}

#[cfg(all(not(target_arch = "wasm32"), feature = "reqwest"))]
#[tracing::instrument]
pub fn get_reqwest_client(timeout: std::time::Duration) -> Result<reqwest::Client, reqwest::Error> {
    #[cfg(any(feature = "native-tls", feature = "rustls-tls"))]
    fn get_certs(
        mut builder: reqwest::ClientBuilder,
        path: &std::ffi::OsString,
    ) -> reqwest::ClientBuilder {
        fn get_cert(path: &Path) -> Result<reqwest::Certificate, anyhow::Error> {
            let is_der = path.extension().is_some_and(|ext| ext == "der");
            let buf = std::fs::read(path)?;
            tracing::info!(
                "Found a custom CA {}. Reading the CA...",
                path.to_string_lossy()
            );
            if is_der {
                Ok(reqwest::Certificate::from_der(&buf)?)
            } else {
                Ok(reqwest::Certificate::from_pem(&buf)?)
            }
        }

        match get_cert(path.as_ref()) {
            Ok(cert) => {
                builder = builder.add_root_certificate(cert);
                tracing::info!(?path, "Added the custom CA");
            }
            Err(err) => {
                tracing::error!(error = %err, "Could not parse the custom CA");
            }
        }
        builder
    }
    #[cfg(not(any(feature = "native-tls", feature = "rustls-tls")))]
    fn get_certs(
        builder: reqwest::ClientBuilder,
        path: std::ffi::OsString,
    ) -> reqwest::ClientBuilder {
        tracing::error!(?path, "Could not load certs, taplo was built without TLS");
        builder
    }

    let mut builder = reqwest::Client::builder().timeout(timeout);
    if let Some(path) = std::env::var_os("TAPLO_EXTRA_CA_CERTS") {
        builder = get_certs(builder, &path);
    }
    builder.build()
}
