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
    switch_oids: Vec<SwitchOid>,
}

impl SwitchOidBuilder {
    pub fn new() -> Self {
        Self {
            switch_oids: SwitchOidBuilder::build(),
        }
    }

    fn build() -> Vec<SwitchOid> {
        let mut switch_oids = Vec::new();

        // Standard IEEE 802.3af PoE MIB OID: pethPsePortAdminEnable
        // 1 = enabled (on), 2 = disabled (off)
        let standard_poe_oid = "1.3.6.1.2.1.105.1.1.1.3.1".to_string();

        switch_oids.push(SwitchOid::new(
            "Aruba".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "Cisco".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "Dell".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "Juniper".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "Netgear".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "TP-Link".to_string(),
            standard_poe_oid.clone(),
            1,
            2,
        ));
        switch_oids.push(SwitchOid::new(
            "Ubiquiti".to_string(),
            standard_poe_oid,
            1,
            2,
        ));

        switch_oids
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

    pub fn get_switch_oid(&self, name: String) -> Option<&String> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| &switch_oid.poe_oid)
    }

    pub fn get_on(&self, name: String) -> Option<i64> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| switch_oid.on)
    }

    pub fn get_off(&self, name: String) -> Option<i64> {
        self.switch_oids
            .iter()
            .find(|switch_oid| switch_oid.name == name)
            .map(|switch_oid| switch_oid.off)
    }
}
