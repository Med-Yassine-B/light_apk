use std::path::PathBuf;
use std::collections::HashMap;
use std::process::Command as cmd;
use crate::commands::build::utils::select_version;

pub fn package_files(workspace_dir:&PathBuf,aapt:&PathBuf,android_home:&PathBuf,configs:&HashMap<String,String>)->Result<(),String>{
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
