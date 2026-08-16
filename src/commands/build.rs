#![allow(warnings)]
use std::io::{self, Error, Write};
use std::ops::Not;
use std::path::PathBuf;
use std::{env, option, path, result};
use crate::commands::commands::Command;
use std::process::Command as cmd;
use std::fs;
use crate::file_parsers::config_parser::parse_config_file;


fn call_back(_args:Vec<String>){
    //current directory
    let workspace_dir:String;
    let Ok(result)=env::current_dir() else{
        eprint!("[ERROR!] Couldnt read current directory path!");
        return;
    };
    let configs=parse_config_file("config.toml");
    //android sdk & ndk paths
    let android_home=env::var("ANDROID_HOME").unwrap_or_else(|_|{
        configs.get("android_home").cloned().unwrap_or_else(||{
            eprintln!("[ERROR] android_home not found please define it in envpath or config.toml");
            String::new()
        })
    });
    if android_home.is_empty(){
        return;
    }
    let ndk=PathBuf::from(&android_home).join("ndk");

    let ndk_version=configs.get("ndk_version").cloned().unwrap_or_else(||{
        match select_ndk_version(&ndk){
            Ok(ndk)=>{
                println!("Consider adding it to the config.toml file ^^");
                return ndk;
            },
            Err(e)=>{
                eprintln!("Failed selecting NDK version: ({})",e);
                return String::new();
            }
        }
        });
    if ndk_version.is_empty(){
        eprintln!("ndk version not found");
        return;
    }
    println!("ndk version selected: {}",ndk_version);




    // let mut output=cmd::new("sh");
    // output.arg("-c").arg("echo $ANDROID_HOME");
    // output.status().expect("failed executing!");



    //build_tools paths
    // let Ok(entries)=fs::read_dir(android_home) else{
    //     println!("Couldnt read dir");
    //     return;
    // };
    // for entry in entries{
    //     if let Ok(entry)=entry {
    //         let path=entry.path();
    //         let entry_type= if path.is_dir(){"[DIR]"}else{"[FILE]"};
    //         println!("{} {}",entry_type,path.to_str().unwrap());
    //     }
    // }

}
const USAGE:&str="";

fn select_ndk_version(ndk_path:&PathBuf)->Result<String,std::io::Error> {
    //ndk-build
    let entries = fs::read_dir(ndk_path)?
                  .filter_map(Result::ok)
                  .collect::<Vec<_>>();
    println!("Available versions:");
    for (mut i,entry) in entries.iter().enumerate(){
        println!("\t{}: {}",i+1,entry.file_name().to_str().unwrap());
        i+=1;
    }

    let option = loop {
        match read_option("Select an NDK version: ") {
            Ok(opt) if opt > 0 && opt <= entries.len() => break opt,
            Ok(opt)=> println!("input out of range"),
            Err(e)=>println!("Invalid input ({}) try again",e),
        }
    };

    let Some(entry)=entries.get(option-1) else{
        eprintln!("could not read selected option");
        return Ok(String::new());
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
