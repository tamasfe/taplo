# associations.fetch_external

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-common\src\schema\associations.rs` (lines 358-381)
- **Role:** internal
- **Stability:** unknown

## Signature

```
async fn fetch_external(&self, index_url: &Url) -> Result<SchemaCatalog, anyhow::Error> {
        let _permit = self.concurrent_requests.acquire().await?;
        match index_url.scheme() {
            "http" | "https" => Ok(self
                .http
                .get(index_url.clone())
                .send()
                .await?
                .json()
                .await?),
            "file" => Ok(serde_json::from_slice(
                &self
                    .env
                    .read_file(
                        self.env
                            .to_file_path_normalized(index_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?
                            .as_ref(),
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    async fn fetch_external(&self, index_url: &Url) -> Result<SchemaCatalog, anyhow::Error> {
        let _permit = self.concurrent_requests.acquire().await?;
        match index_url.scheme() {
            "http" | "https" => Ok(self
                .http
                .get(index_url.clone())
                .send()
                .await?
                .json()
                .await?),
            "file" => Ok(serde_json::from_slice(
                &self
                    .env
                    .read_file(
                        self.env
                            .to_file_path_normalized(index_url)
                            .ok_or_else(|| anyhow!("invalid file path"))?
                            .as_ref(),
                    )
                    .await?,
            )?),
            scheme => Err(anyhow!("the scheme `{scheme}` is not supported")),
        }
    }
```

## Git History

- Commits: 16
- Authors: 5
- Bug fixes: 7
- Features: 4

## Callers

None

## Callees

None
