extern crate bindgen;
use std::env;
use std::path::PathBuf;
 #[cfg(feature="use_sdl3")]
 #[cfg(target_os="windows")]
 const SDL_IMPORT_PATH:&str="C:\\Users\\shimu\\Documents\\SDL3-3.4.2\\include";
 #[cfg(feature="use_sdl3")]
 #[cfg(not(target_os="windows"))]
 const SDL_IMPORT_PATH:&str="/opt/local/include/SDL3";
 #[cfg(feature="use_sdl2")]
 const SDL_IMPORT_PATH:&str="/opt/local/include/SDL2";

 #[cfg(feature="use_sdl3")]
 #[cfg(target_os="windows")]
 const SDL_IMAGE_IMPORT_PATH:&str="C:\\Users\\shimu\\Documents\\SDL3_image-3.4.0\\include\\SDL3_image";
 #[cfg(feature="use_sdl3")]
 #[cfg(not(target_os="windows"))]
 const SDL_IMAGE_IMPORT_PATH:&str="/opt/local/include/SDL3_image";
 #[cfg(feature="use_sdl2")]
 const SDL_IMAGE_IMPORT_PATH:&str="/opt/local/include";

 #[cfg(feature="use_sdl3")]
 #[cfg(target_os="windows")]
 const SDL_TTF_IMPORT_PATH:&str="C:\\Users\\shimu\\Documents\\SDL3_ttf-3.1.0\\include\\SDL3_ttf";
 #[cfg(feature="use_sdl3")]
 #[cfg(not(target_os="windows"))]
 const SDL_TTF_IMPORT_PATH:&str="/opt/local/include/SDL3_ttf";
 #[cfg(feature="use_sdl2")]
 const SDL_TTF_IMPORT_PATH:&str="/opt/local/include";

 #[cfg(feature="use_sdl3")]
 #[cfg(target_os="windows")]
 const SDL_GFX_IMPORT_PATH:&str="C:\\Users\\shimu\\Documents\\SDL3_gfx-master";
 #[cfg(feature="use_sdl3")]
 #[cfg(not(target_os="windows"))]
 const SDL_GFX_IMPORT_PATH:&str="/usr/local/include/SDL3_gfx";
 #[cfg(feature="use_sdl2")]
 const SDL_GFX_IMPORT_PATH:&str="/opt/local/include";

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
        println!("cargo:rustc-link-search={}","/usr/local/lib");
        println!("cargo:rustc-link-lib={}","c++");

    }else{
        println!("cargo:rustc-link-search={}","C:\\Users\\shimu\\Documents\\library");

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
    .clang_arg(format!("-I{}",SDL_IMPORT_PATH))
    .clang_arg(format!("-I{}",SDL_IMAGE_IMPORT_PATH))
    .clang_arg(format!("-I{}",SDL_TTF_IMPORT_PATH))
    .clang_arg(format!("-I{}",SDL_GFX_IMPORT_PATH))
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