# util.get_reqwest_client

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\util.rs` (lines 125-170)
- **Role:** internal
- **Stability:** unknown

## Signature

```
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
```

## Docstring

_No docstring provided._

## Source

```
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
```

## Git History

- Commits: 11
- Authors: 4
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
