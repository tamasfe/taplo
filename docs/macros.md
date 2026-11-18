# Module: macros

**File:** `\crates\taplo\src\formatter\macros.rs`

## Overview

Module `macros` contains 0 analyzed entities.

```
macro_rules! create_options {
    (
        $(#[$attr:meta])*
        pub struct Options {
            $(
                $(#[$field_attr:meta])*
                pub $name:ident: $ty:ty,
            )+
        }
    ) => {
        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        $(#[$attr])*
        pub struct Options {
            $(
                $(#[$field_attr])*
                pub $name: $ty,
            )+
        }

        impl Options {
            pub fn update(&mut self, incomplete: OptionsIncomplete) {
                $(
                    if let Some(v) = incomplete.$name {
                        self.$name = v;
                    }
                )+
            }

            pub fn update_camel(&mut self, incomplete: OptionsIncompleteCamel) {
                $(
                    if let Some(v) = incomplete.$name {
                        self.$name = v;
                    }
                )+
            }

            pub fn update_from_str<S: AsRef<str>, I: Iterator<Item = (S, S)>>(
                &mut self,
                values: I,
            ) -> Result<(), OptionParseError> {
                for (key, val) in values {

                    $(
                        if key.as_ref() == stringify!($name) {
                            self.$name =
                                val.as_ref()
                                    .parse()
                                    .map_err(|error| OptionParseError::InvalidValue {
                                        key: key.as_ref().into(),
                                        error: Box::new(error),
                                    })?;

                            continue;
                        }
                    )+

                    return Err(OptionParseError::InvalidOption(key.as_ref().into()));
                }

                Ok(())
            }
        }

        #[cfg_attr(feature = "schema", derive(JsonSchema))]
        $(#[$attr])*
        #[derive(Defa
```

## Key Components

| Entity | Type | Description |
|--------|------|-------------|
| _none_ | — | — |

## Git History

- Commits: 1
- Authors: 1
- Bug fixes: 0
- Features: 1

## Note

Connect an NVIDIA Nemotron model (Ollama locally or Nebius Serverless) and re-run
`documentor generate` to upgrade these pages to AI-generated narrative docs with
purpose, usage examples, and gotchas.
