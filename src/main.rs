mod commands;
mod file_parsers;
use std::env::args;
use commands::commands::{CmdRegistrer};


fn main() {
    let args:Vec<String> =args().collect();
    let mut registrer:CmdRegistrer=CmdRegistrer::new();

    registrer.add_command(commands::help::get_command());
    registrer.add_command(commands::build::get_command());
    registrer.add_command(commands::init::get_command());

    if args.len()<2{
       println!("try `light_apk --help`");
        return;
    }
    registrer.check_command(args[1].clone(),args);
}
