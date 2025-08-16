use snmp2::Value;

pub fn get_status(status: Value) -> String {
    match status {
        Value::Integer(1) => "On".to_string(),
        Value::Integer(2) => "Off".to_string(),
        _ => "Unknown".to_string(),
    }
}
