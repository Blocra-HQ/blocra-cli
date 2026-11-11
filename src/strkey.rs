//! Decoding of Stellar strkeys (G... account ids), enough to turn a signer's
//! public key into its 32 raw Ed25519 bytes. RFC 4648 base32, one version
//! byte, a 32-byte payload and a CRC16-XModem checksum (little-endian).

const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
const VERSION_ACCOUNT_ID: u8 = 6 << 3; // "G"

fn base32_decode(text: &str) -> Result<Vec<u8>, String> {
    let mut bits = 0u32;
    let mut value = 0u32;
    let mut out = Vec::with_capacity(text.len() * 5 / 8);
    for ch in text.chars() {
        let i = ALPHABET
            .iter()
            .position(|&a| a as char == ch)
            .ok_or_else(|| format!("Invalid character \"{ch}\" in key"))? as u32;
        value = (value << 5) | i;
        bits += 5;
        if bits >= 8 {
            out.push(((value >> (bits - 8)) & 0xff) as u8);
            bits -= 8;
        }
    }
    Ok(out)
}

pub fn crc16_xmodem(bytes: &[u8]) -> u16 {
    let mut crc: u16 = 0;
    for &b in bytes {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 {
                (crc << 1) ^ 0x1021
            } else {
                crc << 1
            };
        }
    }
    crc
}

/// Raw 32-byte Ed25519 public key of a G... account id.
pub fn decode_account_id(strkey: &str) -> Result<[u8; 32], String> {
    if strkey.len() != 56 || !strkey.starts_with('G') {
        return Err("Not a Stellar account id (expected 56 characters starting with G)".into());
    }
    let raw = base32_decode(strkey)?;
    if raw.len() != 35 || raw[0] != VERSION_ACCOUNT_ID {
        return Err("Not a Stellar account id".into());
    }
    let checksum = u16::from_le_bytes([raw[33], raw[34]]);
    if crc16_xmodem(&raw[..33]) != checksum {
        return Err("Key checksum does not match (typo?)".into());
    }
    let mut key = [0u8; 32];
    key.copy_from_slice(&raw[1..33]);
    Ok(key)
}
