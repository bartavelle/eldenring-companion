use clap::Parser;
use packed_struct::PackedStructSlice;
use std::{collections::HashMap, io::{BufRead, BufReader}, path::{Path, PathBuf}};

pub mod bnd4;
pub mod params;
pub mod structs;
pub mod utils;

/*


        var param = Project.AssetLocator.GetAssetPath($@"regulation.bin");
        LoadParamsERFromFile(param);

        string sysParam = Project.AssetLocator.GetAssetPath(@"param\systemparam\systemparam.parambnd.dcx");
        LoadParamsERFromFile(sysParam, false);

    private void LoadParamsERFromFile(string path, bool encrypted = true)
    {
        if (encrypted)
        {
            using BND4 bnd = SFUtil.DecryptERRegulation(path);
            LoadParamFromBinder(bnd, ref _params, out _paramVersion, true);
        }
        else
        {
            using BND4 bnd = BND4.Read(path);
            LoadParamFromBinder(bnd, ref _params, out _, false);
        }
    }

*/

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, help="Path to the regulations file")]
    regulation: PathBuf,
    #[arg(short, long, help="Path to the systemparam.parambnd.dcx file")]
    bnd: Option<PathBuf>,
    #[arg(short, long, help="Source of the ParamDex repository (https://github.com/soulsmods/Paramdex)")]
    paramdex: PathBuf,
}

static REGULATION_KEY: [u8; 32] = [
    0x99, 0xBF, 0xFC, 0x36, 0x6A, 0x6B, 0xC8, 0xC6, 0xF5, 0x82, 0x7D, 0x09, 0x36, 0x02, 0xD6, 0x76, 0xC4, 0x28, 0x92,
    0xA0, 0x1C, 0x20, 0x7F, 0xB0, 0x24, 0xD3, 0xAF, 0x4E, 0x49, 0x3F, 0xEF, 0x99,
];

fn decrypt(key: &[u8; 32], encrypted: &[u8]) -> Vec<u8> {
    let iv = &encrypted[..16];
    let data = &encrypted[16..];
    let cipher = libaes::Cipher::new_256(&REGULATION_KEY);
    cipher.cbc_decrypt(iv, data)
}

fn load_names(dir: &Path, name: &str) -> anyhow::Result<HashMap<u32, String>> {
    let mut dir = dir.to_owned();
    dir.push("ER");
    dir.push("Names");
    dir.push(name);
    let f = std::fs::File::open(dir)?;
    let b = BufReader::new(f);
    let mut out = HashMap::new();
    for l in b.lines() {
        let l = l?;
        match l.split_once(' ') {
            Some((rawid, nm)) => {
                let id = rawid.parse()?;
                out.insert(id, nm.to_string());
            },
            None => panic!("malformed name line {l}")
        }
    }
    Ok(out)
    
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // let dec_prm = std::fs::read(args.parambnd)?;
    // let raw_prm = bnd4::decompress(dec_prm);
    // let parambnd = bnd4::BND4::parse(raw_prm).unwrap();

    let weapons = load_names(&args.paramdex, "EquipParamWeapon.txt")?;

    let enc_reg = std::fs::read(args.regulation)?;
    let dec_reg = decrypt(&REGULATION_KEY, &enc_reg);
    let raw_reg = bnd4::decompress(dec_reg);
    let regulations = bnd4::BND4::parse(raw_reg).unwrap();
    let raw_equip_param_weapon = regulations.get_file("EquipParamWeapon").unwrap();
    let equip_param_weapon = params::Params::from_bytes(raw_equip_param_weapon)?;

    let struct_size = structs::equip_param_weapon::EQUIP_PARAM_WEAPON_ST::packed_bytes_size(None)?;
    for ridx in 0..equip_param_weapon.row_count() {
        let (rid, rdata) = equip_param_weapon.raw_row(ridx);
        let eqpr = structs::equip_param_weapon::EQUIP_PARAM_WEAPON_ST::unpack_from_slice(&rdata[..struct_size])?;
        match weapons.get(&rid) {
            None => println!("!!{rid}!!"),
            Some(nm) => println!("{nm} [{rid}]")
        }
        println!("  {eqpr:?}");
        println!("  pad: {:x?}", &rdata[struct_size..]);
    }

    // println!("row_size: {}", equip_param_weapon.row_size);
    // println!(
    //     "struct_size: {:?}",
    //     structs::equip_param_weapon::EQUIP_PARAM_WEAPON_ST::packed_bytes_size(None)
    // );
    Ok(())
}
