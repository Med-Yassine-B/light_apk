mod utils;
mod build_native;
mod stage;
mod package;

use std::path::PathBuf;
use std::env;
use build_native::build_native_libs;
use stage::stage_files;
use package::package_files;
use crate::commands::commands::Command;
use crate::file_parsers::config_parser::parse_config_file;
use crate::commands::build::utils::select_version;

fn call_back(_args:Vec<String>){
    //current directory
    // let workspace_dir:String;
    let Ok(workspace_dir)=env::current_dir() else{
        eprintln!("[ERROR!] Couldnt read current directory path!");
        return;
    };
    let configs=parse_config_file("config.toml");

    let android_home=configs.get("android_home").cloned().unwrap_or_else(||{
        env::var("ANDROID_HOME").unwrap_or_else(|_|{
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

    let mut ndk_from_config:bool=true;
    let ndk_path=configs.get("android_ndk").cloned().unwrap_or_else(||{
        ndk_from_config=false;
        android_home_path.join("ndk").to_owned().to_str().unwrap_or_else(||{
            return "";
        }).to_string()
        });
    let ndk:PathBuf;
    if ndk_path.is_empty(){
        eprintln!("[ERROR] couldnt select ndk/");
        return;
    }else{
        ndk=PathBuf::from(ndk_path);
    }
    if let Err(e)=build_native_libs(&ndk,&configs,&ndk_from_config){
        eprintln!("{}",e);
        return;
    };
    if let Err(e)=stage_files(&workspace_dir){
        eprintln!("{}",e);
        return;
    };


    let build_tools_path=PathBuf::from(&android_home).join("build-tools");
    if !build_tools_path.exists(){
        eprintln!("[ERROR] Couldnt find build-tools/, make sure it's installed!");
        return;
    }
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
    if let Err(e)=package_files(&workspace_dir, &build_tools.join("aapt"),&PathBuf::from(&android_home),&configs){
        eprintln!("{}",e);
        return;
    }


}
const USAGE:&str="";

pub fn get_command()->Command{
    Command { command: "build".to_string(),_usage:USAGE.to_string(), callback: call_back }
}
