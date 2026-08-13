use std::{env::current_dir};
use crate::commands::commands::Command;
use include_dir::{include_dir,Dir};

static TEMPLATE_DIR:Dir<'static> =include_dir!("$CARGO_MANIFEST_DIR/src/template");

fn call_back(_args:Vec<String>){
    if let Ok(work_space)=current_dir(){
        println!("Generating template...");
        TEMPLATE_DIR.extract(work_space).expect("Failed generating template!");
    }
}

const USAGE:&str="";

pub fn get_command()->Command{
    Command{command:"init".to_string(),_usage:USAGE.to_string(),callback:call_back}
}
