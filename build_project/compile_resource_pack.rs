use std::path::PathBuf;

use crate::build_project::png_to_opa::png_file_to_opa_file;

pub fn compile_resource_pack(input_resource_pack: PathBuf, output_resource_pack: PathBuf) {
    // make sure that cargo reruns this script if the input resource pack changes
    println!(
        "cargo:rerun-if-changed={}",
        input_resource_pack.to_str().unwrap()
    );
    // also rerun if the output resource pack changes
    println!(
        "cargo:rerun-if-changed={}",
        output_resource_pack.to_str().unwrap()
    );

    // create output_resource_pack directory if it doesn't exist
    std::fs::create_dir_all(output_resource_pack.clone()).unwrap();

    // iterate through all files in input_resource_pack
    for entry in std::fs::read_dir(input_resource_pack).unwrap() {
        let path = entry.unwrap().path();

        // skip .DS_Store files
        if path.file_name().unwrap().to_str().unwrap() == ".DS_Store" {
            continue;
        }
        // if its a folder then recursively call this function
        else if path.is_dir() {
            compile_resource_pack(
                path.clone(),
                output_resource_pack.join(path.file_name().unwrap()),
            );
        }
        // if the file ends with .png then convert it to .opa
        else if path.extension().unwrap() == "png" {
            png_file_to_opa_file(
                path.clone(),
                output_resource_pack.join(
                    path.file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace(".png", ".opa"),
                ),
            );
        }
        // else just copy the file
        else {
            std::fs::copy(
                path.clone(),
                output_resource_pack.join(path.file_name().unwrap()),
            )
                .unwrap();
        }
    }
}
