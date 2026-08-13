use crate::commands::commands::Command;
// use crate::commands::commands::

fn call_back(_args:Vec<String>){
    println!("
Usage: light_apk [OPTIONS] COMMAND [ARGS]...

Options:
--help  Show this message and exit.

Commands:
init   Initialize an APK workspace using a template.
build  Build an unsigned APK from the workspace directory.
sign   Align and sign an unsigned APK using apksigner (v2/v3 signatures).
        ");
}
const USAGE:&str="";

pub fn get_command()->Command{
    Command { command: "--help".to_string(),_usage:USAGE.to_string(), callback: call_back }
}
