use std::path::PathBuf;
use std::fs;
use std::io::{Write,stdin,stdout};

pub fn select_version(path:&PathBuf)->Result<String,String> {

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
    stdout().flush().unwrap();
    stdin().read_line(&mut option).expect("failed to read input :( ");
    option.trim().parse::<usize>().map_err(|_| "please inter a valid integer".to_string())
}
