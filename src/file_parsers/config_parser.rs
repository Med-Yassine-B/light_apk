use std::{collections::HashMap, fs};
pub fn parse_config_file(file:&str)->HashMap<String,String>{

    let Ok(data)=fs::read_to_string(file) else{
        eprintln!("Couldnt open {}",file);
        return HashMap::new();
    };

    data.lines().filter_map(|line|{
        let trimed=line.trim();
        if trimed.is_empty() || trimed.starts_with("#"){
            return None;
        }
        let (key,value)=trimed.split_once("=")?;
        Some((key.trim().to_string(),value.trim().to_string()))
    }).collect()

}

#[test]
#[cfg(test)]
fn test_parse_config_file(){
    const FILE:&str="
    ANDROID_HOME=/opt/android-sdk/
    ANDROID_NDK=/opt/android-ndk/
    ";
    let config=parse_config_file(FILE);

    let ndk=config.get("ANDROID_NDK").map_or("0", |s|{s.as_str()});
    assert_eq!(ndk,"/opt/android-ndk/");

    let sdk=config.get("ANDROID_HOME").map_or("0", |s|{s.as_str()});

    assert_eq!(sdk,"/opt/android-sdk/");
}
