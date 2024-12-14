use sails_client_gen::ClientGenerator;
use std::{
    env,
    fs,
    path::PathBuf,
};

fn main() {
    // Build contract to get .opt.wasm
    sails_rs::build_wasm();

    // if the env var exists, it will not generate the contract idl and client
    if env::var("__GEAR_WASM_BUILDER_NO_BUILD").is_ok() {
        return;
    }

    // Path where the file "Cargo.toml" is located (points to the root of the project)
    // 'CARGO_MANIFEST_DIR' specifies this directory in en::var
    let cargo_toml_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Path where the client and idl files will be generated 
    // 'OUT_DIR' points to a temporary directory used by the compiler 
    // to store files generated at compile time
    let outdir_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Path where the file "extended_vmt.idl" and "extended_vmt_client.rs" 
    // files will be created
    let outdir_idl_path = outdir_path.clone().join("extended_vft.idl");
    let outdir_client_path = outdir_path.clone().join("extended_vft_client.rs");

    // Path where the "idl_and_client" directory will be created, it will stores the idl and
    // client from contract
    let idl_and_client_dir_path = cargo_toml_path.clone().join("idl_and_client");

    // This generate the contract IDL
    sails_idl_gen::generate_idl_to_file::<extended_vft_app::ExtendedVftProgram>(outdir_idl_path.clone())
        .unwrap();

    // Generator of the clients of the contract
    ClientGenerator::from_idl_path(&outdir_idl_path)
        .generate_to(outdir_client_path.clone())
        .unwrap();

    // if directory already exists, it will remove it, to avoid errors
    if idl_and_client_dir_path.exists() {
        match fs::remove_dir_all(idl_and_client_dir_path.clone()) {
            Ok(_) => {},
            Err(e) => println!("Error: {:?}", e),
        }
    }

    // create the "idl_and_client" directory to store the contract idl and client
    let _ = fs::create_dir(idl_and_client_dir_path.clone());

    // Then, copies the client and idl that is in the OUT_DIR path in the "idl_and_client" directory
    fs::copy(outdir_client_path, idl_and_client_dir_path.clone().join("extended_vft_client.rs"))
        .unwrap();

    fs::copy(outdir_idl_path, idl_and_client_dir_path.join("extended_vft.idl"))
        .unwrap();
}
