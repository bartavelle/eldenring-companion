use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::utils::{assert_char, assert_u32, reverse_bits};

use super::bnd4::{BinderFileHeader, BND4};

#[derive(Clone, Copy)]
struct Format {
    inner: u8,
}

impl Format {
    fn new(b: u8, bit_big_endian: bool) -> Self {
        assert!(!bit_big_endian);
        Self { inner: reverse_bits(b) }
    }

    fn has_long_offsets(&self) -> bool {
        self.inner & 0b0001_0000 != 0
    }
    fn has_compression(&self) -> bool {
        self.inner & 0b0010_0000 != 0
    }
    fn has_ids(&self) -> bool {
        self.inner & 0b0000_0010 != 0
    }
    fn has_names(&self) -> bool {
        self.inner & 0b0000_1100 != 0
    }
}

#[derive(Debug)]
pub struct BND3 {
    pub version: String,
    data: Vec<u8>,
    files: Vec<BinderFileHeader>,
}

impl From<BND3> for BND4 {
    fn from(value: BND3) -> Self {
        Self {
            version: value.version,
            data: value.data,
            files: value.files,
        }
    }
}

impl BND3 {
    pub fn parse(data: Vec<u8>) -> anyhow::Result<Self> {
        let mut cur = Cursor::new(&data);
        assert_char(b"BND3", &mut cur);
        let mut raw_version = [0; 8];
        cur.read_exact(&mut raw_version)?;
        let version = String::from_utf8_lossy(&raw_version).to_string();
        let bit_big_endian = data[0xe] != 0;
        let format = Format::new(cur.read_u8()?, bit_big_endian);
        let big_endian = cur.read_u8()? != 0;
        let _bit_big_endian = cur.read_u8()?;
        assert_eq!(cur.read_u8()?, 0);
        assert!(!big_endian);
        assert!(!bit_big_endian);

        let file_count = cur.read_u32::<LittleEndian>()?;
        let _eof_header = cur.read_u32::<LittleEndian>()?;
        let unk18 = cur.read_u32::<LittleEndian>()?;
        assert!(unk18 == 0 || unk18 == 0x80000000);
        assert_u32(0, &mut cur);

        let mut headers = Vec::new();
        for _ in 0..file_count {
            let bfh = header_from_reader(&mut cur, format, true, &data)?;
            headers.push(bfh);
        }

        Ok(Self {
            version,
            data,
            files: headers,
        })
    }

    pub fn file_names(&self) -> Vec<&str> {
        self.files
            .iter()
            .map(|f| f.name.as_deref().unwrap_or_default())
            .collect()
    }

    pub fn get_file<'a>(&'a self, name: &str) -> Option<&'a [u8]> {
        let f = self
            .files
            .iter()
            .find(|f| f.name.as_ref().map(|f| f.contains(name)).unwrap_or(false))?;
        Some(&self.data[f.offset..f.offset + f.size])
    }
}

fn header_from_reader<R: Read>(
    cur: &mut R,
    format: Format,
    unicode: bool,
    data: &[u8],
) -> anyhow::Result<BinderFileHeader> {
    let flags = reverse_bits(cur.read_u8()?);
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);
    assert_eq!(cur.read_u8()?, 0);

    let size = cur.read_u32::<LittleEndian>()? as usize;
    let offset = if format.has_long_offsets() {
        cur.read_u64::<LittleEndian>()? as usize
    } else {
        cur.read_u32::<LittleEndian>()? as usize
    };
    let id = if format.has_ids() {
        Some(cur.read_u32::<LittleEndian>()?)
    } else {
        None
    };
    let name = if format.has_names() {
        if unicode {
            let offset = cur.read_u32::<LittleEndian>()? as usize;
            let length = (0..).find(|&n| data[offset + n] == 0).unwrap();
            Some(String::from_utf8_lossy(&data[offset..offset + length]).to_string())
        } else {
            todo!("JIS")
        }
    } else {
        None
    };
    let uncompressed_size = if format.has_compression() {
        Some(cur.read_u32::<LittleEndian>()? as usize)
    } else {
        None
    };
    Ok(BinderFileHeader {
        _flags: flags,
        size,
        _uncompressed_size: uncompressed_size,
        offset,
        _id: id,
        name,
    })
}
