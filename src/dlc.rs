use error::*;
use dlc_decrypter::{PkgData, decrypt_dlc};
use std::fs::File;
use std::io::Read;

pub fn read_dlc(path: &str) -> Result<PkgData> {
    // connection data to jdownloader
    let app_name = "pylo";
    let dec_key = b"cb99b5cbc24db398";
    let dec_iv = b"9bc24cb995cb8db3";

    // read the .dlc file
    let mut file = File::open(&path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    // decrypt the file
    Ok(decrypt_dlc(data, &app_name, dec_key, dec_iv)?)
}