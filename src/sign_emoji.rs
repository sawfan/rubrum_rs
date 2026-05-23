use crate::sign::Sign;
use crate::sign::Sign::*;

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use toml::Value;

static SIGN_SYMBOLS: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let config_path = Path::new("config/sign_symbols.toml");
    let contents = fs::read_to_string(config_path).unwrap_or_default();
    let value = contents
        .parse::<Value>()
        .unwrap_or(Value::Table(Default::default()));

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

pub fn sign_symbol_text(sign: &Sign) -> String {
    let key = format!("{:?}", sign);

    if let Some(s) = SIGN_SYMBOLS.get(&key).cloned() {
        return s;
    }

    // Fall back to built-in Unicode symbols if no config entry is present.
    match sign {
        Aries => "♈".to_owned(),
        Taurus => "♉".to_owned(),
        Gemini => "♊".to_owned(),
        Cancer => "♋".to_owned(),
        Leo => "♌".to_owned(),
        Virgo => "♍".to_owned(),
        Libra => "♎".to_owned(),
        Scorpio => "♏".to_owned(),
        Sagittarius => "♐".to_owned(),
        Capricorn => "♑".to_owned(),
        Aquarius => "♒".to_owned(),
        Pisces => "♓".to_owned(),
    }
}
