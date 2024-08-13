use std::{
    collections::BTreeMap,
    io::{Cursor, Seek},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::utils::read_utf16;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Version {
    DemonsSouls,
    DS1,
    DS3,
}

#[derive(Default, Debug)]
pub struct Fmg {
    pub entries: BTreeMap<u32, String>,
}

impl Fmg {
    pub fn load(buffer: &[u8]) -> anyhow::Result<Self> {
        let mut cur = Cursor::new(buffer);
        assert_eq!(cur.read_u8()?, 0);
        let big_endian = cur.read_u8()? != 0;
        assert!(!big_endian);
        let version = match cur.read_u8()? {
            0 => Version::DemonsSouls,
            1 => Version::DS1,
            2 => Version::DS3,
            v => anyhow::bail!("unsupported version {v}"),
        };
        assert_eq!(cur.read_u8()?, 0);

        let file_size = cur.read_u32::<LittleEndian>()?;
        assert_eq!(cur.read_u8()?, 1);
        assert_eq!(
            cur.read_u8()?,
            if let Version::DemonsSouls = version { 0xff } else { 0x00 }
        );
        assert_eq!(cur.read_u8()?, 0);
        assert_eq!(cur.read_u8()?, 0);
        let group_count = cur.read_u32::<LittleEndian>()?;
        let _string_count = cur.read_u32::<LittleEndian>()?;

        let wide = version == Version::DS3;

        if wide {
            assert_eq!(cur.read_u32::<LittleEndian>()?, 0xff);
        }

        let string_offset_offset = if wide {
            cur.read_u64::<LittleEndian>()?
        } else {
            cur.read_u32::<LittleEndian>()? as u64
        };
        let nxt = if wide {
            cur.read_u64::<LittleEndian>()?
        } else {
            cur.read_u32::<LittleEndian>()? as u64
        };
        assert_eq!(nxt, 0);

        let mut groups = Vec::new();

        struct GroupInfo {
            offset_index: u32,
            first_id: u32,
            last_id: u32,
        }

        for _ in 0..group_count {
            groups.push(GroupInfo {
                offset_index: cur.read_u32::<LittleEndian>()?,
                first_id: cur.read_u32::<LittleEndian>()?,
                last_id: cur.read_u32::<LittleEndian>()?,
            });
            if wide {
                assert_eq!(cur.read_u32::<LittleEndian>()?, 0x0);
            }
        }

        let mut entries = BTreeMap::new();

        for group in groups {
            cur.seek(std::io::SeekFrom::Start(
                string_offset_offset + group.offset_index as u64 * if wide { 8 } else { 4 },
            ))?;
            for id in group.first_id..=group.last_id {
                let string_offset = if wide {
                    cur.read_u64::<LittleEndian>()?
                } else {
                    cur.read_u32::<LittleEndian>()? as u64
                };
                if string_offset > 0 {
                    let entry = read_utf16(&buffer[string_offset as usize..])?;
                    entries.insert(id, entry);
                }
            }
        }
        Ok(Self { entries })
    }

    pub fn merge(self, other: Self) -> Self {
        let mut e = self.entries;
        e.extend(other.entries);
        Self { entries: e }
    }
}
