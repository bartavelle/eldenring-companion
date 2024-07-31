use std::path::Path;

use crate::erformat::bnd4;

static REGULATION_KEY: [u8; 32] = [
    0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02, 0xD6, 0x76, 0xC4, 0x28, 0x92,
    0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E, 0x49, 0x3F, 0xEF, 0x99,
];

fn decrypt(key: &[u8; 32], encrypted: &[u8]) -> Vec<u8> {
    let iv = &encrypted[..16];
    let data = &encrypted[16..];
    let cipher = libaes::Cipher::new_256(key);
    cipher.cbc_decrypt(iv, data)
}

pub fn load_regulation(path: &Path) -> anyhow::Result<bnd4::BND4> {
    let enc_reg = std::fs::read(path)?;
    let dec_reg = decrypt(&REGULATION_KEY, &enc_reg);
    let raw_reg = bnd4::decompress(dec_reg);
    bnd4::BND4::parse(raw_reg)
}
