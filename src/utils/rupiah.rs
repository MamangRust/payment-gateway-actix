

pub fn rupiah_format(digit: &str) -> String {
    // Try to parse the input string to an f64
    match digit.parse::<f64>() {
        Ok(digit_number) => format!("Rp.{:.0}", digit_number),
        Err(_) => "Rp 0".to_string(),
    }
}