use std::path::PathBuf;
use std::collections::HashMap;
use std::process::Command as cmd;
use crate::commands::build::utils::select_version;

pub fn build_native_libs(ndk:&PathBuf,configs:&HashMap<String,String>,ndk_from_config:&bool)->Result<(),String>{
// let ndk=PathBuf::from(&android_home).join("ndk");
if ! ndk.exists(){
    return Err(String::from("[ERROR] Couldnt find ndk/ make sure it's installed!"));
}
let ndk_build:PathBuf;
if !ndk_from_config{
    let ndk_version=configs.get("ndk_version").cloned().unwrap_or_else(||{
        println!("Selecting NDK version!");
        match select_version(&ndk){
            Ok(ndk)=>{
                println!("Consider adding it to the config.toml file ^^");
                return ndk;
            },
            Err(e)=>{
                eprintln!("[ERROR] Failed selecting NDK version: ({})",e);
                return String::new();
            }
        }
        });

    if ndk_version.is_empty(){
        return Err(String::from("ndk version not found"));
    }
    println!("ndk version selected: {}",ndk_version);
    ndk_build=ndk.join(ndk_version).join("ndk-build");
}else{
    ndk_build=ndk.clone().join("ndk-build");
}

if  !ndk_build.exists(){
    return Err(String::from("[ERROR] Couldnt find ndk-build, make sure it's installed!"));
}
//build the binary
println!("[INFO] Compiling native binary with ndk_build");
let mut cmd_output=cmd::new(&ndk_build);
println!("{}",ndk_build.to_str().unwrap());
let status=cmd_output.status().expect("[ERROR] Failed runing ndk-build!");
if !status.success(){
    return Err(String::from("[ERROR] Failed compiling native code!"));
}
Ok(())
}
