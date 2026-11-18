# environment.from

- **Type:** function
- **Language:** rust
- **File:** `\crates\taplo-wasm\src\environment.rs` (lines 163-211)
- **Role:** internal
- **Stability:** unknown

## Signature

```
fn from(val: JsValue) -> Self {
        Self {
            js_now: js_sys::Reflect::get(&val, &JsValue::from_str("js_now"))
                .unwrap()
                .into(),
            js_env_var: js_sys::Reflect::get(&val, &JsValue::from_str("js_env_var"))
                .unwrap()
                .into(),
            js_env_vars: js_sys::Reflect::get(&val, &JsValue::from_str("js_env_vars"))
                .unwrap()
                .into(),
            js_atty_stderr: js_sys::Reflect::get(&val, &JsValue::from_str("js_atty_stderr"))
                .unwrap()
                .into(),
            js_on_stdin: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stdin"))
                .unwrap()
                .into(),
            js_on_stdout: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stdout"))
                .unwrap()
                .into(),
            js_on_stderr: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stderr"))
                .unwrap()
                .into(),
            js_glob_files: js_sys::Reflect::get(&val, &JsValue::from_str("js_glob_files"))
                .unwrap()
                .into(),
            js_read_file: js_sys::Reflect::get(&val, &JsValue::from_str("js_read_file"))
                .unwrap()
                .into(),
            js_write_file: js_sys::Reflect::get(&val, &JsValue::from_str("js_write_file"))
                .unwrap()
                .into(),
            js_to_file_path: js_sys::Reflect::get(&val, &JsValue::from_str("js_to_file_path"))
                .unwrap()
                .into(),
            js_is_absolute: js_sys::Reflect::get(&val, &JsValue::from_str("js_is_absolute"))
                .unwrap()
                .into(),
            js_cwd: js_sys::Reflect::get(&val, &JsValue::from_str("js_cwd"))
                .unwrap()
                .into(),
            js_find_config_file: js_sys::Reflect::get(
                &val,
                &JsValue::from_str("js_find_config_file"),
            )
            .unwrap()
            .into(),
        }
    }
```

## Docstring

_No docstring provided._

## Source

```
    fn from(val: JsValue) -> Self {
        Self {
            js_now: js_sys::Reflect::get(&val, &JsValue::from_str("js_now"))
                .unwrap()
                .into(),
            js_env_var: js_sys::Reflect::get(&val, &JsValue::from_str("js_env_var"))
                .unwrap()
                .into(),
            js_env_vars: js_sys::Reflect::get(&val, &JsValue::from_str("js_env_vars"))
                .unwrap()
                .into(),
            js_atty_stderr: js_sys::Reflect::get(&val, &JsValue::from_str("js_atty_stderr"))
                .unwrap()
                .into(),
            js_on_stdin: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stdin"))
                .unwrap()
                .into(),
            js_on_stdout: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stdout"))
                .unwrap()
                .into(),
            js_on_stderr: js_sys::Reflect::get(&val, &JsValue::from_str("js_on_stderr"))
                .unwrap()
                .into(),
            js_glob_files: js_sys::Reflect::get(&val, &JsValue::from_str("js_glob_files"))
                .unwrap()
                .into(),
            js_read_file: js_sys::Reflect::get(&val, &JsValue::from_str("js_read_file"))
                .unwrap()
                .into(),
            js_write_file: js_sys::Reflect::get(&val, &JsValue::from_str("js_write_file"))
                .unwrap()
                .into(),
            js_to_file_path: js_sys::Reflect::get(&val, &JsValue::from_str("js_to_file_path"))
                .unwrap()
                .into(),
            js_is_absolute: js_sys::Reflect::get(&val, &JsValue::from_str("js_is_absolute"))
                .unwrap()
                .into(),
            js_cwd: js_sys::Reflect::get(&val, &JsValue::from_str("js_cwd"))
                .unwrap()
                .into(),
            js_find_config_file: js_sys::Reflect::get(
                &val,
                &JsValue::from_str("js_find_config_file"),
            )
            .unwrap()
            .into(),
        }
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
