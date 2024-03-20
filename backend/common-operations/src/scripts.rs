use secp256k1::{PublicKey, Secp256k1};
use std::str::FromStr;

//  multisignature script
pub fn generate_multisig_script(pub_keys: Vec<&str>, required_signatures: u8) -> Result<String, &'static str> {
    // Validation of public keys and required signatures
    if pub_keys.len() < required_signatures as usize {
        return Err("Number of public keys must be greater than or equal to the required number of signatures");
    }

    // Parse public keys
    let secp = Secp256k1::new();
    let mut pub_keys_parsed = Vec::new();
    for pub_key_str in pub_keys {
        let pub_key = PublicKey::from_str(pub_key_str).map_err(|_| "Invalid public key")?;
        pub_keys_parsed.push(pub_key);
    }

    //  multisig script
    let script = format!(
        "OP_{} {} OP_CHECKMULTISIG",
        required_signatures,
        pub_keys_parsed
            .iter()
            .map(|pub_key| pub_key.serialize().to_vec().to_hex())
            .collect::<Vec<String>>()
            .join(" ")
    );

    Ok(script)
}
