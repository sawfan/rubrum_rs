use rubrum::{Body, EndpointKey};

fn main() {
    // Parse from canonical string.
    let sun = EndpointKey::parse("sun").unwrap();
    assert_eq!(sun, EndpointKey::Body(Body::Sun));

    // Convert back to the canonical key.
    println!("sun canonical key = {}", sun.canonical_key());

    // Stable ordering uses canonical strings.
    let mut keys = vec![EndpointKey::parse("pluto").unwrap(), sun];
    keys.sort();
    println!("sorted keys = {:?}", keys);

    // Serde uses the canonical string representation.
    let json = serde_json::to_string(&sun).unwrap();
    println!("sun as json = {json}");

    // Unknown keys are rejected.
    assert!(EndpointKey::parse("definitely_not_real").is_err());
}
