use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::utils::{assert_char, assert_u32, read_utf16, reverse_bits};

pub fn decompress(bytes: Vec<u8>) -> Vec<u8> {
    match &bytes[..4] {
        b"DCP\0" => todo!(),
        b"DCX\0" => match &bytes[40..44] {
            b"ZSTD" => decompress_zstd(&bytes),
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

fn decompress_krak(bytes: &[u8]) -> anyhow::Result<Vec<u8>> {
    let mut cur = Cursor::new(bytes);
    let (uncompressed, compressed) = validate_dcx(&mut cur);
    assert_char(b"KRAK", &mut cur);
    assert_u32(32, &mut cur);
    let compression_level = cur.read_u8()?;
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
    let end = start + compressed as usize;
    let mut decompressed = vec![0; uncompressed as usize];
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
    fn is_name1(&self) -> bool {
        self.inner == 0b0000_0100
    }

    fn file_header_size(&self) -> u64 {
        0x10 + if self.has_long_offsets() { 8 } else { 4 }
            + if self.has_compression() { 8 } else { 0 }
            + if self.has_ids() { 4 } else { 0 }
            + if self.has_names() { 4 } else { 0 }
            + if self.is_name1() { 8 } else { 0 }
    }
}

#[derive(Debug)]
pub struct BND4 {
    unk4: bool,
    unk5: bool,
    pub version: String,
    data: Vec<u8>,
    files: Vec<BinderFileHeader>,
}

impl BND4 {
    pub fn parse(data: Vec<u8>) -> anyhow::Result<Self> {
        let mut cur = Cursor::new(&data);
        assert_char(b"BND4", &mut cur);
        let unk4 = cur.read_u8()? != 0;
        let unk5 = cur.read_u8()? != 0;
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_u8()?, 0);

        assert_eq!(cur.read_u8()?, 0);
        let big_endian = cur.read_u8()? != 0;
        let bit_big_endian = cur.read_u8()? == 0;
        assert_eq!(cur.read_u8()?, 0);

        assert!(!big_endian);
        assert!(!bit_big_endian);

        let file_count = cur.read_u32::<LittleEndian>()?;
        assert_eq!(cur.read_u64::<LittleEndian>()?, 0x40);

        let mut raw_version = [0; 8];
        cur.read_exact(&mut raw_version)?;
        let version = String::from_utf8_lossy(&raw_version).to_string();
        let file_header_size = cur.read_u64::<LittleEndian>()?;
        let _header_end = cur.read_u64::<LittleEndian>()?;

        let unicode = cur.read_u8()? != 0;
        let format = Format::new(cur.read_u8()?, bit_big_endian);
        let extended = cur.read_u8()?;
        assert!([0, 1, 4, 0x80].contains(&extended));
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_u32::<LittleEndian>()?, 0);

        if extended == 4 {
            let _hashtable_offset = cur.read_u64::<LittleEndian>()?;
            // ignore
        } else {
            assert_eq!(cur.read_u32::<LittleEndian>()?, 0);
        }

        assert_eq!(file_header_size, format.file_header_size());

        let mut headers = Vec::new();
        for _ in 0..file_count {
            let bfh = BinderFileHeader::from_reader(&mut cur, format, unicode, &data)?;
            headers.push(bfh);
        }

        Ok(Self {
            unk4,
            unk5,
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

#[derive(Debug)]
struct BinderFileHeader {
    flags: u8,
    size: usize,
    uncompressed_size: Option<usize>,
    offset: usize,
    id: Option<u32>,
    name: Option<String>,
}

impl BinderFileHeader {
    fn from_reader<R: Read>(cur: &mut R, format: Format, unicode: bool, data: &[u8]) -> anyhow::Result<Self> {
        let flags = reverse_bits(cur.read_u8()?);
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_i32::<LittleEndian>()?, -1);
        let size = cur.read_u64::<LittleEndian>()? as usize;
        let uncompressed_size = if format.has_compression() {
            Some(cur.read_u64::<LittleEndian>()? as usize)
        } else {
            None
        };
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
                Some(read_utf16(&data[offset..])?)
            } else {
                todo!("JIS")
            }
        } else {
            None
        };
        assert!(!format.is_name1());
        Ok(Self {
            flags,
            size,
            uncompressed_size,
            offset,
            id,
            name,
        })
    }
}
