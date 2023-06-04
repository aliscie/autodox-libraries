use ic_cdk::export::candid::{
    export_service
};
use ic_cdk_macros::*;
use std::fs::{create_dir_all, write};
use std::path::{PathBuf};

pub fn generated_candid(current_dir: PathBuf) {
    #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
    fn export_candid() -> String {
        export_service!();
        __export_service()
    }

    let folder_name = current_dir
        .file_name()
        .expect("Failed to get folder name")
        .to_str()
        .expect("Failed to convert folder name to string");

    let dir = PathBuf::from(&current_dir);
    match create_dir_all(&dir) {
        Ok(_) => println!("Successfully created directory"),
        Err(e) => println!("Failed to create directory: {}", e),
    }
    let file = format!("{}.did", folder_name);
    let res = write(dir.join(file), export_candid());
    println!("-------- Wrote to {:?}", dir);
    println!("-------- res {:?}", res);
}
