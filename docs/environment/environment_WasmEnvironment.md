# environment.WasmEnvironment

- **Type:** struct
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 145-160)
- **Role:** internal
- **Stability:** unknown

## Signature

```
pub(crate) struct WasmEnvironment {
    js_now: Function,
    js_env_var: Function,
    js_env_vars: Function,
    js_atty_stderr: Function,
    js_on_stdin: Function,
    js_on_stdout: Function,
    js_on_stderr: Function,
    js_glob_files: Function,
    js_read_file: Function,
    js_write_file: Function,
    js_to_file_path: Function,
    js_is_absolute: Function,
    js_cwd: Function,
    js_find_config_file: Function,
}
```

## Docstring

_No docstring provided._

## Source

```
pub(crate) struct WasmEnvironment {
    js_now: Function,
    js_env_var: Function,
    js_env_vars: Function,
    js_atty_stderr: Function,
    js_on_stdin: Function,
    js_on_stdout: Function,
    js_on_stderr: Function,
    js_glob_files: Function,
    js_read_file: Function,
    js_write_file: Function,
    js_to_file_path: Function,
    js_is_absolute: Function,
    js_cwd: Function,
    js_find_config_file: Function,
}
```

## Git History

- Commits: 6
- Authors: 2
- Bug fixes: 3
- Features: 2

## Callers

None

## Callees

None
