use snmp2::Value;

pub fn get_status(status: Value) -> String {
    match status {
        Value::Integer(1) => "On".to_string(),
        Value::Integer(2) => "Off".to_string(),
        _ => "Unknown".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_status_on() {
        assert_eq!(get_status(Value::Integer(1)), "On");
    }

    #[test]
    fn test_get_status_off() {
        assert_eq!(get_status(Value::Integer(2)), "Off");
    }

    #[test]
    fn test_get_status_unknown_value() {
        assert_eq!(get_status(Value::Integer(3)), "Unknown");
    }

    #[test]
    fn test_get_status_zero() {
        assert_eq!(get_status(Value::Integer(0)), "Unknown");
    }
}
