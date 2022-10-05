#![allow(non_snake_case)]
// #![windows_subsystem = "windows"]

fn main() {
    // let target = dirs::download_dir().expect("Can't find Download folder :(");
    let target = std::path::PathBuf::from("D:\\");
    let mut fcount = 0;
    for file in glob::glob(target.clone().join("*.*").as_os_str().to_str().unwrap()).unwrap() {
        let file = file.expect("File ERRORR  AGFAIEGHEIGHG");
        if file.is_dir() || file.ends_with("desktop.ini") { continue; };

        let ext = file.extension().and_then(std::ffi::OsStr::to_str).unwrap();
        let base_name = file.file_stem().and_then(std::ffi::OsStr::to_str).unwrap();

        let this_target = target.clone().join(ext);

        std::fs::create_dir_all(this_target.clone()).unwrap();

        let mut n: u8 = 1;
        let mut this_actual_target = this_target.clone().join([base_name.clone(), ".", ext.clone()].join(""));

        while this_actual_target.exists() {
            let nam_buff = [base_name.clone(), " (", &n.to_string(), ").", ext.clone()].join("");
            this_actual_target.pop();
            this_actual_target.push(nam_buff);
            n += 1;
        }

        std::fs::rename(file.clone(), this_actual_target.clone()).expect("Error moving file!");
        fcount += 1;
        println!("Moved from [{:?}] to [{:?}]", file.display(), this_actual_target.display());
    }
    println!("Moved {:?} files.", fcount);
}