extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main(){
    if cfg!(not(feature="non_bindings")){
        println!("cargo::rerun-if-changed={}","./target/cmake/SDL2_ffi/libSDL2_ffi.a");
        println!("cargo:rustc-link-search={}","./target/cmake");
        println!("cargo:rustc-link-lib={}","SDL2_ffi");

    }else{
        println!("cargo::rerun-if-changed={}","./Cargo.toml");

    }
    if cfg!(not(target_os="windows")){
        println!("cargo:rustc-link-search={}","/opt/local/lib");
        println!("cargo:rustc-link-lib={}","c++");

    }
    if cfg!(feature="use_sdl3"){
        println!("cargo:rustc-link-lib={}","SDL3");
        println!("cargo:rustc-link-lib={}","SDL3_Image");
        println!("cargo:rustc-link-lib={}","SDL3_ttf");
        println!("cargo:rustc-link-lib={}","SDL3_gfx");
    }else{
        println!("cargo:rustc-link-lib={}","SDL2");
        println!("cargo:rustc-link-lib={}","SDL2_Image");
        println!("cargo:rustc-link-lib={}","SDL2_ttf");
        println!("cargo:rustc-link-lib={}","SDL2_gfx");

    }


    let bindings = bindgen::Builder::default()
    .header("src/imports.h")
    .clang_arg("-I/opt/local/include")
    .clang_arg(
        if cfg!(feature="use_sdl3"){"-I/opt/local/include/SDL3"}
        else{"-I/opt/local/include/SDL2"})
    .clang_arg("-I/opt/local/include/SDL3_image")
    .clang_arg("-I/opt/local/include/SDL3_ttf")
    .clang_arg("-I/usr/local/include/SDL3_gfx")
    .clang_arg(
        if cfg!(feature="use_sdl3"){"-DUSE_SDL3"}
        else{"-DUSE_SDL2"})
    //.clang_arg("-std=c++17")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
    .generate()
    .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
    .write_to_file(out_path.join("cpp_imports.rs"))
    .expect("Couldn't write bindings!");
}