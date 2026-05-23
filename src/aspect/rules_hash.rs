use crate::aspect::AspectRules;

/// Compute a stable (enough) hash for an aspect rules config.
///
/// This is meant for cache invalidation, not cryptographic use.
pub fn rules_hash_hex(rules: &AspectRules) -> Option<String> {
    // TOML serialization makes a convenient stable-ish canonical form for config.
    //
    // If serialization fails, just skip hashing.
    let Ok(toml_value) = toml::Value::try_from(rules) else {
        return None;
    };
    let s = toml_value.to_string();

    Some(fnv1a_64_hex(s.as_bytes()))
}

fn fnv1a_64_hex(bytes: &[u8]) -> String {
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x100000001b3;

    let mut hash = FNV_OFFSET;
    for b in bytes {
        hash ^= *b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    format!("{hash:016x}")
}
