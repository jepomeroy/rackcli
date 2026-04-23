pub struct SwitchOid {
    name: String,
    poe_oid: String,
    on: i64,
    off: i64,
}

impl SwitchOid {
    fn new(name: String, poe_oid: String, on: i64, off: i64) -> SwitchOid {
        SwitchOid {
            name,
            poe_oid,
            on,
            off,
        }
    }
}

pub struct SwitchOidBuilder {
    switch_oids: [SwitchOid; 7],
}

impl SwitchOidBuilder {
    pub fn new() -> Self {
        // Standard IEEE 802.3af PoE MIB OID: pethPsePortAdminEnable
        // 1 = enabled (on), 2 = disabled (off)
        let standard_poe_oid = "1.3.6.1.2.1.105.1.1.1.3.1";

        let switch_oids = [
            SwitchOid::new("Aruba".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("Cisco".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("Dell".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("Juniper".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("Netgear".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("TP-Link".to_string(), standard_poe_oid.to_string(), 1, 2),
            SwitchOid::new("Ubiquiti".to_string(), standard_poe_oid.to_string(), 1, 2),
        ];

        Self { switch_oids }
    }

    pub fn get_oid_names(&self) -> Vec<String> {
        self.switch_oids
            .iter()
            .map(|switch_oid| switch_oid.name.clone())
            .collect()
    }

    pub fn get_oid_name(&self, index: usize) -> String {
        self.switch_oids[index].name.clone()
    }

    pub fn get_switch_oid(&self, name: &str) -> Option<&String> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| &switch_oid.poe_oid)
    }

    pub fn get_on(&self, name: &str) -> Option<i64> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| switch_oid.on)
    }

    pub fn get_off(&self, name: &str) -> Option<i64> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| switch_oid.off)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STANDARD_OID: &str = "1.3.6.1.2.1.105.1.1.1.3.1";

    #[test]
    fn test_get_oid_names_count() {
        assert_eq!(SwitchOidBuilder::new().get_oid_names().len(), 7);
    }

    #[test]
    fn test_get_oid_names_contains_all_brands() {
        let names = SwitchOidBuilder::new().get_oid_names();
        for brand in &["Aruba", "Cisco", "Dell", "Juniper", "Netgear", "TP-Link", "Ubiquiti"] {
            assert!(names.contains(&brand.to_string()), "missing brand: {}", brand);
        }
    }

    #[test]
    fn test_get_switch_oid_known_brand() {

        let builder = SwitchOidBuilder::new();
        let oid = builder.get_switch_oid("Netgear");
        assert_eq!(oid, Some(&STANDARD_OID.to_string()));
    }

    #[test]
    fn test_get_switch_oid_unknown_brand() {
        assert_eq!(SwitchOidBuilder::new().get_switch_oid("Unknown"), None);
    }

    #[test]
    fn test_get_on_returns_one() {
        for brand in &["Aruba", "Cisco", "Dell", "Juniper", "Netgear", "TP-Link", "Ubiquiti"] {
            assert_eq!(SwitchOidBuilder::new().get_on(brand), Some(1), "brand: {}", brand);
        }
    }

    #[test]
    fn test_get_off_returns_two() {
        for brand in &["Aruba", "Cisco", "Dell", "Juniper", "Netgear", "TP-Link", "Ubiquiti"] {
            assert_eq!(SwitchOidBuilder::new().get_off(brand), Some(2), "brand: {}", brand);
        }
    }

    #[test]
    fn test_get_on_unknown_brand() {
        assert_eq!(SwitchOidBuilder::new().get_on("Unknown"), None);
    }

    #[test]
    fn test_get_oid_name_by_index() {
        assert_eq!(SwitchOidBuilder::new().get_oid_name(0), "Aruba");
    }
}
