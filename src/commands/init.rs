use std::{env::current_dir, fs::{self}};
use crate::commands::commands::Command;
use include_dir::{Dir, include_dir};

static TEMPLATE_DIR:Dir<'static> =include_dir!("$CARGO_MANIFEST_DIR/src/template");

fn call_back(_args:Vec<String>){


    if let Ok(work_space)=current_dir(){
        let Ok(mut entries)=fs::read_dir(&work_space) else{
            eprintln!("[ERROR] failed ropening workspace!");
            return;
        };
        if !entries.next().is_none(){
            eprintln!("[ERROR] Workspace should be empty!");
            return;
        }
        println!("Generating template...");
        TEMPLATE_DIR.extract(work_space).expect("Failed generating template!");
        println!("Template generated [✓]");
    }
}

const USAGE:&str="";

pub fn get_command()->Command{
    Command{command:"init".to_string(),_usage:USAGE.to_string(),callback:call_back}
}
