use ethereum_types::{H256, U256};
use serde::{
    de::{Error as DeError, Unexpected},
    Deserialize, Deserializer,
};

pub fn deserialize_h256_from_hex<'de, D>(d: D) -> Result<H256, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(d)?;
    let value = value.trim_start_matches("0x");

    let hex_chars_len = value.len();
    let expected_hex_length = H256::len_bytes() * 2;

    let parsed_bytes: Vec<u8> = if hex_chars_len == expected_hex_length {
        hex::decode(value).map_err(|err| DeError::custom(format!("Invalid hex: {}", err)))?
    } else if hex_chars_len < expected_hex_length {
        let mut padded_hex = str::repeat("0", expected_hex_length - hex_chars_len);
        padded_hex.push_str(value);
        hex::decode(&padded_hex).map_err(|err| DeError::custom(format!("Invalid hex: {}", err)))?
    } else {
        return Err(DeError::invalid_value(
            Unexpected::Str(value),
            &"Hex too long",
        ));
    };

    Ok(H256::from_slice(&parsed_bytes))
}

pub fn deserialize_vec_u256_from_dec<'de, D>(d: D) -> Result<Vec<U256>, D::Error>
where
    D: Deserializer<'de>,
{
    let values: Vec<String> = Vec::deserialize(d)?;

    values
        .iter()
        .map(|value| U256::from_dec_str(value))
        .collect::<Result<Vec<U256>, _>>()
        .map_err(|err| DeError::custom(format!("Invalid integer: {}", err)))
}
