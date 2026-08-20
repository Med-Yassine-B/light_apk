use std::collections::HashMap;
use std::io::{self,Write};
use std::path::PathBuf;
use std::env;
use crate::commands::commands::Command;
use std::process::Command as cmd;
use std::fs;
use crate::file_parsers::config_parser::parse_config_file;


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

fn build_native_libs(ndk:&PathBuf,configs:&HashMap<String,String>,ndk_from_config:&bool)->Result<(),String>{
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
fn stage_files(workspace_dir:&PathBuf)->Result<(),String>{
    //stage the binary in dist/stage/lib/
    let stage_dir=&workspace_dir.join("dist/stage") ;

    let lib_dir=stage_dir.join("lib");
    if let Err(e)=fs::create_dir_all(&lib_dir){
        return Err(format!("[ERROR] Failed creating lib/ ({})",e));
    }
    let build_lib=&workspace_dir.join("libs");
    let Ok(build_lib_entries)=fs::read_dir(build_lib) else{
        return Err(String::from("[Error] Failed reading libs/ entries!"));
    };

    for arch_entry in build_lib_entries{
        let Ok(arch_entry)=arch_entry else{
            continue;
        };
        let staging_path=lib_dir.join(arch_entry.file_name());
        if !staging_path.exists(){
            if let Err(e)=fs::create_dir(&staging_path){
                eprintln!("Failed creating {} ({})",staging_path.to_str().unwrap_or("(Failed reading file name)"),e);
                continue;
            }
        }
        let Ok(lib_entries)=fs::read_dir(arch_entry.path())else {
            continue;
        };
        for lib_entry in lib_entries{
            let Ok(lib_entry)=lib_entry else{
                continue;
            };
            let os_file_name=lib_entry.file_name();
            let Some(lib_entry_name)=os_file_name.to_str() else{
                eprintln!("[ERROR]Failed converting arch_entry name to str");
                continue;
            };
            if lib_entry_name.ends_with(".so"){
                let staging_path=staging_path.join(lib_entry_name);
                if let Err(_)=fs::copy(lib_entry.path(), &staging_path){
                    eprintln!("[ERROR] Failed copying {} to {}",lib_entry.path().to_str().unwrap_or("(Failed reading file name)"),staging_path.to_str().unwrap_or("(Failed reading file name)"));
                }
            }
        }

    }
    Ok(())
}
fn package_files(workspace_dir:&PathBuf,aapt:&PathBuf,android_home:&PathBuf,configs:&HashMap<String,String>)->Result<(),String>{
    if !aapt.exists(){
        return Err(String::from("[ERROR] Couldnt find aapt, make sure it's isntalled!"));
    }
    // let dist_path=workspace_dir.join("dist");
    let android_platforms_path=android_home.join("platforms");
    let android_platforms_version=configs.get("android_platrorm_version").cloned().unwrap_or_else(||{
        println!("Selecting android_platforms version");
        match select_version(&android_platforms_path){
            Ok(opt)=>{
                println!("Consider adding it to the config.toml file ^^");
                return opt;
            },
            Err(e)=>{
                eprintln!("[ERROR] Failed selecting android_platform version: ({})",e);
                return String::new();
            }
        };
    });
    let android_jar=android_platforms_path.join(android_platforms_version).join("android.jar");
    if ! android_jar.exists(){
        return Err(String::from("[ERROR] couldnt find android.jar"));
    }
    let manifest=workspace_dir.join("AndroidManifest.xml");
    if !manifest.exists(){
        return Err(String::from("[ERROR] AndroidManifest.xml not found"));
    }

    let unsigned_apk_path=workspace_dir.join("dist/unsigned.app.apk");
    let staged_lib_path=workspace_dir.join("dist/stage");

    let mut cmd_output=cmd::new(aapt);
    cmd_output.arg("package").arg("-f")
              .arg("-M").arg(manifest)
              .arg("-I").arg(android_jar)
              .arg("-F").arg(unsigned_apk_path)
              .arg("--shared-lib").arg(staged_lib_path);
    let status=&cmd_output.status().expect("Failed running aapt");
    if !status.success(){
        return Err(String::from("[ERROR] Failed packaging the app"));
    }
    println!("[INFO] app packaged succesfuly");


    Ok(())
}

fn select_version(path:&PathBuf)->Result<String,String> {
    //ndk-build
    let Ok(entries) = fs::read_dir(path) else{
        return Err(String::from("[ERROR] Couldnt open folder!"));
    };
    let entries = entries.filter_map(Result::ok)
                  .collect::<Vec<_>>();
    if entries.is_empty(){
        eprintln!();
        return Err(String::from("[ERROR] No versions found!"));
    }
    println!("Available versions:");
    for (i,entry) in entries.iter().enumerate(){
        println!("\t{}: {}",i+1,entry.file_name().to_str().unwrap());
    }

    let option = loop {
        match read_option("Select an version: ") {
            Ok(opt) if opt > 0 && opt <= entries.len() => break opt,
            Ok(_opt)=> println!("input out of range"),
            Err(e)=>println!("Invalid input ({}) try again",e),
        }
    };

    let Some(entry)=entries.get(option-1) else{
        return Ok(String::from("could not read selected option"));
    };   
    let version=entry.file_name().to_str().unwrap_or("").to_string();

    return Ok(version);
}
fn read_option(message:&str)->Result<usize,String>{
    let mut option:String=String::new();
    print!("{}",message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut option).expect("failed to read input :( ");
    option.trim().parse::<usize>().map_err(|_| "please inter a valid integer".to_string())
}


pub fn get_command()->Command{
    Command { command: "build".to_string(),_usage:USAGE.to_string(), callback: call_back }
}
