#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_integer_strings() {
        assert_eq!(convert_string_to_number("123").unwrap().to_string(), "123");
        assert_eq!(convert_string_to_number("0").unwrap().to_string(), "0");
        assert_eq!(
            convert_string_to_number("-456").unwrap().to_string(),
            "-456"
        );
    }

    #[test]
    fn test_valid_float_non_zero_strings_with_zero_fractional_part() {
        assert_eq!(
            convert_string_to_number("123.0").unwrap().to_string(),
            "123"
        );

        assert_eq!(
            convert_string_to_number("456.0").unwrap().to_string(),
            "456"
        );
    }

    #[test]
    fn test_valid_float_zero_with_zero_fractional_part() {
        assert_eq!(convert_string_to_number("0.0").unwrap().to_string(), "0");
    }

    #[test]
    fn test_valid_float_strings_with_non_zero_fractional_part() {
        assert_eq!(
            convert_string_to_number("123.45").unwrap().to_string(),
            "123.45"
        );
        assert_eq!(
            convert_string_to_number("0.678").unwrap().to_string(),
            "0.678"
        );
        assert_eq!(
            convert_string_to_number("-456.789").unwrap().to_string(),
            "-456.789"
        );
    }

    #[test]
    fn test_invalid_numeric_strings() {
        assert!(convert_string_to_number("123a").is_err());
        assert!(convert_string_to_number("12.3.4").is_err());
        assert!(convert_string_to_number("abc").is_err());
    }

    #[test]
    fn test_edge_cases() {
        // assert_eq!(
        //     convert_string_to_number("1e10").unwrap().to_string(),
        //     "10000000000"
        // );
        assert_eq!(convert_string_to_number(".5").unwrap().to_string(), "0.5");
        assert_eq!(convert_string_to_number("-0").unwrap().to_string(), "0");
    }

    #[test]
    fn test7() {
        assert_eq!(convert_string_to_number("5.").unwrap().to_string(), "5");
    }
}

fn convert_string_to_number(s: &str) -> Result<Box<dyn std::fmt::Display>, String> {
    match s.parse::<f64>() {
        Ok(f) => {
            // Check if the float is effectively an integer
            if f == f.trunc() {
                // If the fractional part is zero, return as integer
                match s.parse::<i64>() {
                    Ok(i) => Ok(Box::new(i)),
                    Err(_) => Err("Failed to convert string to integer".to_string()),
                }
            } else {
                // Otherwise, return as float
                Ok(Box::new(f))
            }
        }
        Err(_) => Err("Failed to convert string to float".to_string()),
    }
}
