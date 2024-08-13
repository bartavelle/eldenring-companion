use packed_struct::PackedStructSlice;
use std::collections::BTreeMap;

pub mod bnd3;
pub mod bnd4;
pub mod compression;
pub mod fmg;
pub mod params;

pub fn load_params<A, B, F, K>(reg: &bnd4::BND4, name: &str, convert: F) -> anyhow::Result<BTreeMap<K, B>>
where
    K: Ord,
    A: PackedStructSlice,
    F: Fn(u32, Option<&str>, A) -> (K, B),
{
    let raw = reg.get_file(name).unwrap();
    let paramed = params::Params::from_bytes(raw)?;
    let mut out = BTreeMap::new();
    for ridx in 0..paramed.row_count() {
        let (rid, nm, rdata) = paramed.raw_row(ridx);
        let clc = A::unpack_from_slice(rdata)?;
        let (k, row) = convert(rid, nm, clc);
        out.insert(k, row);
    }
    Ok(out)
}

pub fn load_params_filter<A, B, F, K>(reg: &bnd4::BND4, name: &str, convert: F) -> anyhow::Result<BTreeMap<K, B>>
where
    K: Ord,
    A: PackedStructSlice,
    F: Fn(u32, Option<&str>, A) -> Option<(K, B)>,
{
    let raw = reg.get_file(name).unwrap();
    let paramed = params::Params::from_bytes(raw)?;
    let mut out = BTreeMap::new();
    for ridx in 0..paramed.row_count() {
        let (rid, nm, rdata) = paramed.raw_row(ridx);
        let clc = A::unpack_from_slice(rdata)?;
        if let Some((k, row)) = convert(rid, nm, clc) {
            out.insert(k, row);
        }
    }
    Ok(out)
}
