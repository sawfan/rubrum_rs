use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

/// Load and deserialize a TOML config struct using the same hybrid approach used by
/// `load_toml_string_map`:
///
/// - Prefer a runtime file path (ignored if unreadable/missing)
/// - Fall back to embedded TOML defaults
/// - If both fail to parse, fall back to `T::default()`
pub fn load_toml_config<T>(runtime_path: &Path, embedded_toml: &str) -> T
where
    T: for<'de> serde::Deserialize<'de> + Default,
{
    let contents = fs::read_to_string(runtime_path).unwrap_or_else(|_| embedded_toml.to_owned());

    toml::from_str(&contents)
        .unwrap_or_else(|_| toml::from_str::<T>(embedded_toml).unwrap_or_else(|_| T::default()))
}

/// Load a simple TOML mapping from a `[symbols]` (or similarly named) table.
///
/// Expected TOML structure:
///
/// ```toml
/// [symbols]
/// SomeKey = "SomeValue"
/// ```
///
/// Notes:
/// - Keys are treated as opaque strings. Prefer canonical keys (e.g. `Sign::canonical_key()`).
/// - If the runtime config file can't be read, the embedded TOML is used.
/// - If TOML parsing fails, an empty map is returned.
pub fn load_toml_string_map(
    runtime_path: &Path,
    embedded_toml: &str,
    table_name: &str,
) -> HashMap<String, String> {
    // Prefer a runtime override from the current working directory (useful for apps), but fall
    // back to the embedded rubrum defaults when the consumer doesn't provide config files.
    let contents = fs::read_to_string(runtime_path).unwrap_or_else(|_| embedded_toml.to_owned());

    let value = contents.parse::<Value>().unwrap_or_else(|_| {
        embedded_toml
            .parse::<Value>()
            .unwrap_or(Value::Table(Default::default()))
    });

    let mut map = HashMap::new();
    if let Value::Table(tbl) = value {
        if let Some(Value::Table(symbols)) = tbl.get(table_name) {
            for (k, v) in symbols {
                if let Value::String(s) = v {
                    map.insert(k.clone(), s.clone());
                }
            }
        }
    }

    map
}

// NOTE: We intentionally do not provide a generic `lazy_toml_string_map(...) -> Lazy<_>` helper here.
//
// `once_cell::sync::Lazy::new` is a `const fn` that requires a concrete initializer type (`F`). Returning
// a `Lazy<T, F>` from a function would force us to name the closure type, which isn't possible on stable Rust.
//
// Instead, define `static FOO: Lazy<HashMap<_, _>> = Lazy::new(|| load_toml_string_map(...));` at each call site.
