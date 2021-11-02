use std::{env, fs, io, path::{Path, PathBuf}};

fn main() {
    let copy_folder_list = vec!["optolith-data", "assets"];

    let manifest_dir = get_manifest_path();
    let out_dir = get_output_path();

    for src in copy_folder_list {
        let source_dir = manifest_dir.join(src);
        let target_dir = out_dir.join(src);
        
        println!("cargo:warning=copy {} -> {}", source_dir.to_str().unwrap(), target_dir.to_str().unwrap());
        let res = copy_dir(&source_dir, &target_dir);
        if res.is_err() {
            println!("cargo:warning=copy failed");
            eprintln!("{:#?}",res);
        }
    }    
}

fn get_output_path() -> PathBuf {
    //<root or manifest path>/target/<profile>/
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string).join("target").join(build_type);
    return PathBuf::from(path);
}

fn get_manifest_path() -> PathBuf {    
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    return PathBuf::from(manifest_dir_string);
}

fn copy_dir(src: &Path, dest: &Path) -> io::Result<()> {
    if !dest.exists() {
        fs::create_dir(&dest)?;
    }
    for entry in src.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let file_type = entry.file_type()?;
        if file_type.is_file() {
            fs::copy(path, dest.join(file_name))?;
        } else if file_type.is_dir() {
            let dest_dir = dest.join(file_name);
            fs::create_dir(&dest_dir)?;
            copy_dir(&path, &dest_dir)?;
        } else {
            unreachable!("unknown file type");
        }
    }
    Ok(())
}
