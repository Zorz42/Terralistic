use crate::png_to_opa::png_to_opa;

pub fn compile_resource_pack(input_resource_pack: String, output_resource_pack: String) {
    // make sure that cargo reruns this script if the input resource pack changes
    println!("cargo:rerun-if-changed={}", input_resource_pack);
    // also rerun if the output resource pack changes
    println!("cargo:rerun-if-changed={}", output_resource_pack);

    // make sure input_resource_pack is a directory and ends with a slash
    // also make sure output_resource_pack is a directory and ends with a slash
    let mut input_resource_pack = input_resource_pack;
    if !input_resource_pack.ends_with("/") {
        input_resource_pack.push_str("/");
    }
    let mut output_resource_pack = output_resource_pack;
    if !output_resource_pack.ends_with("/") {
        output_resource_pack.push_str("/");
    }
    // create output_resource_pack directory if it doesn't exist
    std::fs::create_dir_all(output_resource_pack.clone()).unwrap();

    // iterate through all files in input_resource_pack
    for entry in std::fs::read_dir(input_resource_pack.clone()).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let path_str = path.to_str().unwrap();

        // skip .DS_Store files
        if path_str.ends_with(".DS_Store") {
            continue;
        }

        // if its a folder then recursively call this function
        if path.is_dir() {
            compile_resource_pack(path_str.to_string(), output_resource_pack.clone() + path.file_name().unwrap().to_str().unwrap() + "/");
        }

        // if the file ends with .png then convert it to .opa
        else if path_str.ends_with(".png") {
            png_to_opa(path_str.to_string(), output_resource_pack.clone() + path.file_name().unwrap().to_str().unwrap().replace(".png", ".opa").as_str());
        }

        // else just copy the file
        else {
            std::fs::copy(path_str, output_resource_pack.clone() + path.file_name().unwrap().to_str().unwrap()).unwrap();
        }
    }
}