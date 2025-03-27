pub mod hex_serde_u64 {
    use serde::{self, Serializer, Deserializer, Deserialize};

    // Serializace: převede u64 na hexadecimální řetězec s prefixem "0x".
    pub fn serialize<S>(addr: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", addr))
    }

        // Deserializace: přečte řetězec, odstraní prefix "0x" (pokud existuje) a převede na u64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let trimmed = s.trim_start_matches("0x");
        u64::from_str_radix(trimmed, 16)
            .map_err(serde::de::Error::custom)
    }
}

pub mod hex_serde_u16 {
    use serde::{self, Serializer, Deserializer, Deserialize};

    // Serializace: převede u64 na hexadecimální řetězec s prefixem "0x".
    pub fn serialize<S>(addr: &u16, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{:x}", addr))
    }

        // Deserializace: přečte řetězec, odstraní prefix "0x" (pokud existuje) a převede na u64.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<u16, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let trimmed = s.trim_start_matches("0x");
        u16::from_str_radix(trimmed, 16)
            .map_err(serde::de::Error::custom)
    }
}