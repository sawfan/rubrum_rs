use super::*;

pub fn trunc_and_fract(f: f64) -> TruncFractArray {
    let trunc = f.trunc();
    let fract = f.fract();
    [trunc, fract]
}

pub fn truncfract_to_tuple(a: TruncFractArray) -> (f64, f64) {
    match a {
        [w, f] => (w, f),
    }
}

pub fn format_degrees_str(degrees: f64) -> String {
    format!("{}°", degrees)
}

pub fn format_minutes_str(minutes: f64) -> String {
    format!("{}′", minutes)
}

pub fn format_seconds_str(seconds: f64) -> String {
    format!("{}″", seconds)
}

pub fn format_degrees_minutes_str(degrees: f64, minutes: f64) -> String {
    format!(
        "{}{}",
        format_degrees_str(degrees),
        format_minutes_str(minutes)
    )
}

pub fn format_degrees_minutes_seconds_str(degrees: f64, minutes: f64, seconds: f64) -> String {
    format!(
        "{}{}{}",
        format_degrees_str(degrees),
        format_minutes_str(minutes),
        format_seconds_str(seconds)
    )
}
