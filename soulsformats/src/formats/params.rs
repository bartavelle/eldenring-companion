use std::io::Read;

use byteorder::{LittleEndian, ReadBytesExt};

use crate::utils::{assert_u32, read_ascii, read_utf16};

struct Row {
    id: u32,
    name: Option<String>,
    index: u64,
}

pub struct Params<'t> {
    rows: Vec<Row>,
    pub row_size: u64,
    data: &'t [u8],
}

const FLAG01: u8 = 0b0000_0001;
const INT_DATA_OFFSET: u8 = 0b0000_0010;
const LONG_DATA_OFFSET: u8 = 0b0000_0100;
const OFFSET_PARAM_TYPE: u8 = 0b1000_0000;

const UNICODE_ROW_NAME: u8 = 0b0000_0001;

impl<'t> Params<'t> {
    pub fn from_bytes(bts: &'t [u8]) -> anyhow::Result<Self> {
        let be = bts[0x2c] == 0xff;
        assert!(!be);
        let format_2d = bts[0x2d];
        let format_2e = bts[0x2e];
        let _param_format_version = bts[0x2f];
        let mut cur = std::io::Cursor::new(bts);
        let _string_offset = cur.read_u32::<LittleEndian>()?;
        let _ = cur.read_u16::<LittleEndian>()?;
        let _unk06 = cur.read_u16::<LittleEndian>()?;
        let _data_version = cur.read_u16::<LittleEndian>()?;
        let row_count = cur.read_u16::<LittleEndian>()?;

        let (param_type, _param_type_offset) = if (format_2d & OFFSET_PARAM_TYPE) != 0 {
            assert_u32(0, &mut cur);
            let param_type_offset = cur.read_u64::<LittleEndian>()?;
            let mut buf = [0; 0x14];
            cur.read_exact(&mut buf)?;
            if buf.iter().any(|&x| x != 0) {
                anyhow::bail!("non empty buffer")
            }
            (None, Some(param_type_offset))
        } else {
            let mut o = vec![0; 0x20];
            cur.read_exact(&mut o)?;
            (Some(read_ascii(&o)), None)
        };
        if let Some(param_type) = param_type {
            eprintln!("TODO, handle param type {param_type}")
        }
        let _some_format = cur.read_u32::<LittleEndian>()?;
        if (format_2d & FLAG01) != 0 && (format_2d & INT_DATA_OFFSET) != 0 {
            let _data_start = cur.read_u32::<LittleEndian>()?;
            eprintln!("TODO: data_start 0x{_data_start:x}");
            assert_u32(0, &mut cur);
            assert_u32(0, &mut cur);
            assert_u32(0, &mut cur);
        } else if (format_2d & LONG_DATA_OFFSET) != 0 {
            let _data_start = cur.read_u64::<LittleEndian>()?;
            eprintln!("TODO: data_start (B) 0x{_data_start:x}");
            assert_u32(0, &mut cur);
            assert_u32(0, &mut cur);
        }
        let mut rows = Vec::new();
        for _ in 0..row_count {
            let (id, dataindex, name_offset) = if (format_2d & LONG_DATA_OFFSET) != 0 {
                let id = cur.read_u32::<LittleEndian>()?;
                assert_u32(0, &mut cur);
                let data_index = cur.read_u64::<LittleEndian>()?;
                let name_offset = cur.read_u64::<LittleEndian>()?;
                (id, data_index, name_offset)
            } else {
                let id = cur.read_u32::<LittleEndian>()?;
                let data_index = cur.read_u32::<LittleEndian>()?;
                let name_offset = cur.read_u32::<LittleEndian>()?;
                (id, data_index as u64, name_offset as u64)
            };
            let name: Option<String> = if name_offset != 0 {
                // does not handle row_count == 1
                if format_2e & UNICODE_ROW_NAME != 0 {
                    read_utf16(&bts[name_offset as usize..]).ok()
                } else {
                    Some(read_ascii(&bts[name_offset as usize..]))
                }
            } else {
                None
            };
            rows.push(Row {
                id,
                name,
                index: dataindex,
            });
        }
        rows.sort_by(|a, b| a.index.cmp(&b.index));
        if rows.len() < 2 {
            anyhow::bail!("table too small")
        }
        let row_size = rows[1].index - rows[0].index;

        for (a, b) in rows.iter().zip(rows.iter().skip(1)) {
            let sz = b.index - a.index;
            assert_eq!(row_size, sz)
        }

        Ok(Self {
            rows,
            row_size,
            data: bts,
        })
    }

    pub fn raw_row(&'t self, idx: usize) -> (u32, Option<&'t str>, &'t [u8]) {
        let rowid = &self.rows[idx];
        let start = rowid.index as usize;
        let end = start + self.row_size as usize;
        (rowid.id, rowid.name.as_deref(), &self.data[start..end])
    }

    pub(crate) fn row_count(&self) -> usize {
        self.rows.len()
    }
}
