use std::env::{current_dir,var};
use std::fs;
use std::path::PathBuf;
use std::process::Command as cmd;
use crate::commands::commands::Command;
use crate::file_parsers::config_parser::parse_config_file;
use crate::commands::build::utils::select_version;

fn call_back(_args:Vec<String>){

    println!("[INFO] Signing ...");
    let Ok(workspace_dir)=current_dir() else{
        eprintln!("[ERROR!] Couldnt read current directory path!");
        return;
    };
    let configs=parse_config_file("config.toml");

    let android_home=configs.get("android_home").cloned().unwrap_or_else(||{
        var("ANDROID_HOME").unwrap_or_else(|_|{
            eprintln!("[ERROR] android_home not found please define it in envpath or config.toml");
            String::new()
        })
    });
    if android_home.is_empty(){
        eprintln!("[ERROR] Failed finding android-sdk, put it in config.toml or env variables");
        return;
    }
    let android_home_path=PathBuf::from(&android_home);
    if ! android_home_path.exists(){
        eprintln!("[ERROR] {} dosent exist as android-home",&android_home);
        return;
    }

    let build_tools_path=android_home_path.join("build-tools");

    let build_tools_version=configs.get("build_tools_version").cloned().unwrap_or_else(||{
        println!("Selecting build_tools version!");
        match select_version(&build_tools_path){
            Ok(build_tools)=>{
                println!("Consider adding it to the config.toml file ^^");
                return build_tools;
            },
            Err(e)=>{
                eprintln!("[ERROR] Failed selecting build-tools version! {}",e);
                return String::new();
            }
        };
    });
    if build_tools_version.is_empty(){
        return;
    }else{
        println!("build-tools version selected: {}",build_tools_version);
    }

    let build_tools=build_tools_path.join(build_tools_version);
    if ! build_tools.exists(){
        eprintln!("[ERROR] build-tools dosent exist!");
        return;
    }
    if let Err(e)=align_apk(&workspace_dir, &build_tools){
        eprintln!("{}",e);
        return;
    }
    println!("[INFO] apk aligned successfuly");

    if _args.len() < 3{
        eprintln!("[ERROR] Please path the keystore path!");
        return;
    }
    let keystore_path=PathBuf::from(_args[2].clone());
    if let Err(e)=sign_apk(&workspace_dir, &build_tools, &keystore_path){
        eprint!("{}",e);
        return;
    }
}
fn align_apk(workspace_dir:&PathBuf,build_tools:&PathBuf)->Result<(),String>{
    let zipalign=build_tools.join("zipalign");
    if ! zipalign.exists(){
        return Err(String::from("[ERROR] zipalign dosent exist!"));
    }
    let unsigned_apk=workspace_dir.join("dist/unsigned.app.apk");
    if ! unsigned_apk.exists(){
        return Err(String::from("[ERROR] didnt find unsigned_apk.app.apk try building the app first!"));
    }
    let aligned_apk=workspace_dir.join("dist/aligned.app.apk");
    let _ = fs::remove_file(&aligned_apk);
    let mut zipalign_cmd=cmd::new(zipalign);
    zipalign_cmd.arg("-v").arg("-p").arg("4")
                .arg(unsigned_apk)
                .arg(aligned_apk);
    let Ok(status)=zipalign_cmd.status() else{
        return Err(String::from("[ERROR] Failed running zipalign!"));
    };
    if ! status.success(){
        return Err(String::from("[ERROR] Failed aligning the apk"));

    }
    Ok(())
}

fn sign_apk(workspace_dir:&PathBuf,build_tools:&PathBuf,keystore_path:&PathBuf)->Result<(),&'static str>{
    let apksigner=build_tools.join("apksigner");
    if ! apksigner.exists(){
        return Err("[ERROR] apksigner dosent exist!");
    }
    let aligned_apk=workspace_dir.join("dist/aligned.app.apk");
    if ! aligned_apk.exists(){
        return Err("[ERROR] aligned.app.apk not found! in dist/");
    }
    let app_apk=workspace_dir.join("dist/app.apk");
    if !keystore_path.exists(){
        return Err("[ERROR] Keystore dosent exist in provided path!");
    }
    let mut apksigner_cmd=cmd::new(apksigner);
    apksigner_cmd.arg("sign")
                 .arg("--ks").arg(keystore_path)
                 .arg("--out").arg(app_apk)
                 .arg(aligned_apk);
    let Ok(status)=apksigner_cmd.status() else{
        return Err("[ERROR] Failed running apksigner!");
    };
    if ! status.success(){
        return Err("[ERROR] Failed signing the apk");
    }

    Ok(())
}

const USAGE:&str="";

pub fn get_command()->Command{
    Command{command:"sign".to_string(),_usage:USAGE.to_string(),callback:call_back}
}
