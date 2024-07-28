use clap::Parser;
use std::{
    path::PathBuf,
};

pub mod bnd4;

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
    #[arg(short, long)]
    regulation: PathBuf,
    #[arg(short, long)]
    parambnd: PathBuf,
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


fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    // let dec_prm = std::fs::read(args.parambnd)?;
    // let raw_prm = bnd4::decompress(dec_prm);
    // let parambnd = bnd4::BND4::parse(raw_prm).unwrap();

    let enc_reg = std::fs::read(args.regulation)?;
    let dec_reg = decrypt(&REGULATION_KEY, &enc_reg);
    let raw_reg = bnd4::decompress(dec_reg);
    let regulations = bnd4::BND4::parse(raw_reg).unwrap();
    Ok(())
}
