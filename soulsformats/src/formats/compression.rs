use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::utils::{assert_char, assert_u32};

pub fn decompress(bytes: Vec<u8>) -> Vec<u8> {
    match &bytes[..4] {
        b"DCP\0" => todo!(),
        b"DCX\0" => match &bytes[40..44] {
            b"ZSTD" => decompress_zstd(&bytes),
            b"DFLT" => decompress_dflt(&bytes).unwrap(),
            b"KRAK" => decompress_krak(&bytes).unwrap(),
            x => todo!("unsupported compression format {}", String::from_utf8_lossy(x)),
        },
        _ => bytes,
    }
}

fn validate_dcx<R: Read>(cur: &mut R) -> (u32, u32) {
    assert_char(b"DCX\0", cur);
    assert_u32(0x11000, cur);
    assert_u32(24, cur);
    assert_u32(36, cur);
    assert_u32(68, cur);
    assert_u32(76, cur);
    assert_char(b"DCS\0", cur);
    let uncompressed = cur.read_u32::<BigEndian>().unwrap();
    let compressed = cur.read_u32::<BigEndian>().unwrap();
    assert_char(b"DCP\0", cur);
    (uncompressed, compressed)
}

fn decompress_zstd(bytes: &[u8]) -> Vec<u8> {
    let mut cur = Cursor::new(bytes);
    let (_, compressed) = validate_dcx(&mut cur);
    assert_char(b"ZSTD", &mut cur);
    assert_u32(32, &mut cur);
    assert_u32(0x15000000, &mut cur);
    assert_u32(0, &mut cur);
    assert_u32(0, &mut cur);
    assert_u32(0, &mut cur);
    assert_u32(0x10100, &mut cur);
    assert_char(b"DCA\0", &mut cur);
    assert_u32(8, &mut cur);
    // resize
    let start = cur.position() as usize;
    let end = start + compressed as usize;
    let mut newcur = Cursor::new(&bytes[start..end]);
    zstd::decode_all(&mut newcur).unwrap()
}

fn decompress_dflt(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut cur = Cursor::new(bytes);
    assert_char(b"DCX\0", &mut cur);
    let _unk04 = cur.read_u32::<BigEndian>()?;
    assert_u32(0x18, &mut cur);
    assert_u32(0x24, &mut cur);
    let _unk10 = cur.read_u32::<BigEndian>()?;
    let _unk14 = cur.read_u32::<BigEndian>()?;
    assert_char(b"DCS\0", &mut cur);
    let uncompressed = cur.read_u32::<BigEndian>()? as usize;
    let compressed = cur.read_u32::<BigEndian>()? as usize;
    assert_char(b"DCP\0", &mut cur);
    assert_char(b"DFLT", &mut cur);
    assert_u32(0x20, &mut cur);
    let _unk30 = cur.read_u8()?;
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_u32(0x0, &mut cur);
    let _unk38 = cur.read_u8()?;
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_u32(0x0, &mut cur);
    assert_u32(0x10100, &mut cur);
    assert_char(b"DCA\0", &mut cur);
    let _compressed_hdr_len = cur.read_u32::<BigEndian>()?;
    assert_eq!(compressed, bytes.len() - 76);
    assert_eq!(cur.read_u8()?, 0x78);
    let _foo = cur.read_u8()?;
    let mut d = flate2::read::DeflateDecoder::new(cur);
    let mut out = Vec::new();
    d.read_to_end(&mut out)?;
    assert_eq!(out.len(), uncompressed);
    Ok(out)
}

fn decompress_krak(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut cur = Cursor::new(bytes);
    let (uncompressed, compressed) = validate_dcx(&mut cur);
    assert_char(b"KRAK", &mut cur);
    assert_u32(32, &mut cur);
    let _compression_level = cur.read_u8()?;
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_u32(0, &mut cur);
    assert_u32(0, &mut cur);
    assert_u32(0, &mut cur);
    assert_u32(0x10100, &mut cur);
    assert_char(b"DCA\0", &mut cur);
    assert_u32(8, &mut cur);
    let start = cur.position() as usize;
    let _end = start + compressed as usize;
    let mut _decompressed = vec![0; uncompressed as usize];
    // let decompressed_size = oodle_safe::decompress(
    //     &bytes[start..end],
    //     &mut decompressed,
    //     None,
    //     Some(oodle_safe::CheckCRC::Yes),
    //     None,
    //     None,
    // );
    // todo!("{compression_level} - {uncompressed} - {decompressed_size:?}")
    todo!()
}
