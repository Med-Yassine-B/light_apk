use std::path::PathBuf;
use std::fs;

pub fn stage_files(workspace_dir:&PathBuf)->Result<(),String>{
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
