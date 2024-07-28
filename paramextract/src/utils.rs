use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

pub fn reverse_bits(i: u8) -> u8 {
    (0..8)
        .map(|n| {
            let a = 1_u8 << n;
            let b = 1_u8 << (7 - n);
            if i & a != 0 {
                b
            } else {
                0
            }
        })
        .sum()
}

pub fn assert_char<R: Read>(c: &[u8; 4], r: &mut R) {
    let mut buf = [0_u8; 4];
    r.read_exact(&mut buf).unwrap();
    assert_eq!(&buf, c);
}

pub fn assert_u32<R: Read>(expected: u32, r: &mut R) {
    let actual = r.read_u32::<BigEndian>().unwrap();
    assert_eq!(expected, actual);
}

pub fn read_utf16(data: &[u8]) -> anyhow::Result<String> {
    let Some(ln) = memchr::memmem::find(data, &[0, 0]) else {
        anyhow::bail!("could not find name end");
    };
    let (front, slice, back) = unsafe { data[0..=ln].align_to::<u16>() };
    if front.is_empty() && back.iter().all(|&x| x == 0) {
        Ok(String::from_utf16(slice)?)
    } else {
        anyhow::bail!("could not align front{front:?} back{back:?}")
    }
}
