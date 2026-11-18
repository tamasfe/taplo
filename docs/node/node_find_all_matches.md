# node.find_all_matches

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo\src\dom\node.rs` (lines 179-245)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub fn find_all_matches(
        &self,
        keys: Keys,
        include_children: bool,
    ) -> Result<impl ExactSizeIterator<Item = (Keys, Node)>, Error> {
        let mut all = self.flat_iter_impl();

        let mut err: Option<Error> = None;

        all.retain(|(k, _)| {
            if k.len() < keys.len() {
                return false;
            }

            let search_keys = keys.clone();
            let keys = k.clone();

            for (search_key, key) in search_keys.iter().zip(keys.iter()) {
                match search_key {
                    KeyOrIndex::Key(search_key) => {
                        let glob = match globset::Glob::new(search_key.value()) {
                            Ok(g) => g.compile_matcher(),
                            Err(glob_err) => {
                                err = Some(QueryError::from(glob_err).into());
                                return true;
                            }
                        };

                        match key {
                            KeyOrIndex::Key(key) => {
                                if !glob.is_match(key.value()) {
                                    return false;
                                }
                            }
                            KeyOrIndex::Index(idx) => {
                                if !glob.is_match(idx.to_string()) {
                                    return false;
                                }
                            }
                        }
                    }
                    KeyOrIndex::Index(search_idx) => match key {
                        KeyOrIndex::Key(_) => {
                            return false;
                        }
                        KeyOrIndex::Index(idx) => {
                            if idx != search_idx {
                                return false;
                            }
                        }
                    },
                }
            }

            true
        });

        if !include_children {
            all.retain(|(k, _)| k.len() == keys.len());
        }

        if let Some(err) = err {
            return Err(err);
        }

        Ok(all.into_iter())
    }
```

## Docstring

_No docstring provided._

## Source

```
    pub fn find_all_matches(
        &self,
        keys: Keys,
        include_children: bool,
    ) -> Result<impl ExactSizeIterator<Item = (Keys, Node)>, Error> {
        let mut all = self.flat_iter_impl();

        let mut err: Option<Error> = None;

        all.retain(|(k, _)| {
            if k.len() < keys.len() {
                return false;
            }

            let search_keys = keys.clone();
            let keys = k.clone();

            for (search_key, key) in search_keys.iter().zip(keys.iter()) {
                match search_key {
                    KeyOrIndex::Key(search_key) => {
                        let glob = match globset::Glob::new(search_key.value()) {
                            Ok(g) => g.compile_matcher(),
                            Err(glob_err) => {
                                err = Some(QueryError::from(glob_err).into());
                                return true;
                            }
                        };

                        match key {
                            KeyOrIndex::Key(key) => {
                                if !glob.is_match(key.value()) {
                                    return false;
                                }
                            }
                            KeyOrIndex::Index(idx) => {
                                if !glob.is_match(idx.to_string()) {
                                    return false;
                                }
                            }
                        }
                    }
                    KeyOrIndex::Index(search_idx) => match key {
                        KeyOrIndex::Key(_) => {
                            return false;
                        }
                        KeyOrIndex::Index(idx) => {
                            if idx != search_idx {
                                return false;
                            }
                        }
                    },
                }
            }

            true
        });

        if !include_children {
            all.retain(|(k, _)| k.len() == keys.len());
        }

        if let Some(err) = err {
            return Err(err);
        }

        Ok(all.into_iter())
    }
```

## Git History

- Commits: 8
- Authors: 6
- Bug fixes: 2
- Features: 1

## Callers

None

## Callees

None
