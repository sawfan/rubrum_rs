use super::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

use crate::Body;

const EMBEDDED_BODY_SYMBOLS: &str = include_str!("../../config/body_symbols.toml");

static BODY_SYMBOLS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    // Prefer a runtime override from the current working directory (useful for apps), but fall
    // back to the embedded rubrum defaults when the consumer doesn't provide config files.
    let config_path = Path::new("config/body_symbols.toml");
    let contents =
        fs::read_to_string(config_path).unwrap_or_else(|_| EMBEDDED_BODY_SYMBOLS.to_owned());

    let value = contents.parse::<Value>().unwrap_or_else(|_| {
        EMBEDDED_BODY_SYMBOLS
            .parse::<Value>()
            .unwrap_or(Value::Table(Default::default()))
    });

    let mut map = HashMap::new();
    if let Value::Table(tbl) = value {
        if let Some(Value::Table(symbols)) = tbl.get("symbols") {
            for (k, v) in symbols {
                if let Value::String(s) = v {
                    map.insert(k.clone(), s.clone());
                }
            }
        }
    }
    map
});

pub fn try_body_symbol_text(body: &Body) -> Option<String> {
    let key = format!("{:?}", body);
    BODY_SYMBOLS.get(&key).cloned()
}

pub fn body_symbol_text(body: &Body) -> String {
    try_body_symbol_text(body).unwrap_or_else(|| body.name().to_owned())
}
