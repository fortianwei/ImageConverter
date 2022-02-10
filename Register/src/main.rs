use std::io;
use winreg::{enums, RegValue};
use winreg::RegKey;
use winreg::enums::RegType::REG_DWORD;
use rust_embed::RustEmbed;
use std::process::Command;
use std::env::temp_dir;


#[derive(RustEmbed)]
#[folder="prebuilt/"]
#[prefix="exe/"]
struct Asset;

fn check_magick_exists() {

    let ret = Command::new("magick").arg("-version").output();
    match ret {
        Err(e) => {
            install_magick()
        },
        Ok(_) => {}
    }
}

fn install_magick() {
    let magick_exe = Asset::get("exe/ImageMagick-7.1.0.exe").unwrap();
    let temp_dir = temp_dir();
    let tmp_path = temp_dir.join("ImageMagick.exe");
    std::fs::write(&tmp_path, magick_exe.data);

    Command::new(&tmp_path).output();
}

fn register(path: &str) -> io::Result<()> {
    println!("starting ...");
    let my_key = RegKey::predef(enums::HKEY_CLASSES_ROOT);
    let (my_key, disposition) = my_key.create_subkey(path)?;
    // let (my_key, disposition) = my_key.create_subkey("Directory\\shell\\runas")?;
    // let (my_key, disposition) = my_key.create_subkey("Directory\\Background\\shell\\runas")?;
    // let (my_key, disposition) = my_key.create_subkey("*\\shell\\jpg2png")?;

    // match disposition {
    //     enums::REG_CREATED_NEW_KEY => println!("== step 1 runas created, succeed."),
    //     enums::REG_OPENED_EXISTING_KEY => println!(" runas already exist, skip 1")
    // }
    my_key.set_value("", &"VS 白底JPG转透明PNG");

    // let keys: Vec<u8> = vec![200,155,99,0];
    // let data = RegValue {vtype: REG_DWORD, bytes: keys};
    // my_key.set_raw_value("ShowBaseOnVelocityId", &data)?;
    let (my_key, disposition) = my_key.create_subkey("command")?;
    // match disposition {
    //     enums::REG_CREATED_NEW_KEY => println!("==step2 runas-command created, succeed."),
    //     enums::REG_OPENED_EXISTING_KEY => println!("=runas command already exists, skip 2")
    // }

    if path.contains("Background") {
        my_key.set_value("", &"VsConverter.exe %V")?;
    }else{
        my_key.set_value("", &"VsConverter.exe %1")?;
    }

    Ok(())
}

fn main() {
    use Asset;
    let converter_exe = Asset::get("exe/VsConverter.exe").unwrap();
    std::fs::write("C:/Windows/VsConverter.exe", converter_exe.data);
    // let (my_key, disposition) = my_key.create_subkey("Directory\\shell\\runas")?;
    // let (my_key, disposition) = my_key.create_subkey("Directory\\Background\\shell\\runas")?;
    // let (my_key, disposition) = my_key.create_subkey("*\\shell\\jpg2png")?;
    let result = register("Directory\\shell\\vsjpg2png");
    register("Directory\\Background\\shell\\vsjpg2png");
    register("*\\shell\\vsjpg2png");

    check_magick_exists();

    match result {
        Ok(()) => println!("OK"),
        Err(error) => {
            println!("add failed {}", error);
        }
    }
}
