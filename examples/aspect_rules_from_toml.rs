use rubrum::*;

const RULES_TOML: &str = r#"
orb_combine = "max"
include_bodies = true
include_angles = true

[[aspects]]
kind = "conjunction"
orb_deg = 8.0

[[aspects]]
kind = "trine"
orb_deg = 6.0

[[aspects]]
kind = "opposition"
orb_deg = 8.0

[[orb_overrides]]
endpoint = "sun"
orb_deg = 10.0

[[orb_overrides]]
endpoint = "pluto"
kind = "conjunction"
orb_deg = 2.0
"#;

fn main() -> anyhow::Result<()> {
    let rules: AspectRules = toml::from_str(RULES_TOML)?;
    rules.validate()?;

    println!("loaded {} aspect rules", rules.aspects.len());
    for rule in &rules.aspects {
        println!(
            "- {} orb {}°",
            rule.kind
                .format_degree_aspect_kind(DegreeAspectKindFormat::Name),
            rule.orb_deg
        );
    }

    Ok(())
}
