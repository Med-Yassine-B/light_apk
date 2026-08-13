#![allow(warnings)]
use std::{env, result};
use crate::commands::commands::Command;
use std::process::Command as cmd;
use std::fs;


fn call_back(_args:Vec<String>){
    //current directory
    let workspace_dir:String;
    let Ok(result)=env::current_dir() else{
        eprint!("[ERROR!] Couldnt read current directory path!");
        return;
    };
    //android sdk & ndk paths
    let Ok(android_home)=env::var("ANDROID_HOME") else{
        println!("ANDROID_HOME not Found!");
        //check config files...
        return;
    };
    let Ok(android_ndk)=env::var("ANDROID_NDK") else{
        println!("ANDROID_NDK not Found!");
        //check config files...
        return;
    };
    println!("ndk:{},home:{}",android_ndk,android_home);

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

pub fn get_command()->Command{
    Command { command: "build".to_string(),_usage:USAGE.to_string(), callback: call_back }
}
