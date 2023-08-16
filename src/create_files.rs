

pub mod create_files{

use std::fs;
// use std::io::prelude::*;
// use std::path::Path;
use std::path::PathBuf;

pub fn create_files(file_directory: &PathBuf, file_name:&String){

    let file_path = format!("{}/{}.vue", file_directory.display(), file_name);

    let  file_contents: &str = "
<script setup></script>
<template></template>
<style></style>
    ";

    fs::write(&file_path, file_contents).unwrap();

    println!("{} created here -> {}",file_name, &file_path);

    }

}
