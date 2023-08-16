//cli takes filename(s) as argument

//get the current working directory

// append the the filenames to the cwd

// create a new file(s) using the new
// directory

//write the scaffold in it

//TODO:
// get App.vue file
// import new component file into App.vue script
// insert new component file into template

// vue component scaffold creator
// create component file
// with
// template
// script
// style

// import into App.vue


mod create_files;

use clap::Parser;
use create_files::create_files::create_files;
use std::env;

/// scaffold component files in VueJS
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// component file names
    file_name: Vec<String>,
}

fn main(){

    let path = env::current_dir().unwrap();


    let args = Args::parse();

    if args.file_name.len() == 1{
        let file_name = args.file_name.into_iter().collect::<String>();
        create_files(&path, &file_name);

        }
    else if args.file_name.len() > 1 {
        for file_names in args.file_name {
            create_files(&path, &file_names);
    }

}


}
