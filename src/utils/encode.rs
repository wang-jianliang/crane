use base64::{engine::general_purpose, Engine as _};

pub fn string_to_base64(s: &String) -> String {
    let encoded: String = general_purpose::STANDARD_NO_PAD.encode(s.as_bytes());
    encoded
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_string_to_base64() {
        assert_eq!(
            "dGhpcyBpcyBhIHRlc3QgY2FzZQ",
            string_to_base64(&String::from("this is a test case"))
        );
    }
}
