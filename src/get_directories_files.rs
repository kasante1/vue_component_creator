/// fetch all files from subdirectories

pub mod file_directories{
    use std::path::Path;
    use walkdir::WalkDir;

        pub fn fetch_file_directories(default_directory: String) -> Vec<String>{

            let mut fetch_files = vec![];

            for entry in WalkDir::new(default_directory).into_iter().filter_map(|e| e.ok()) {

                let path_filename = entry.path().display().to_string();

                if Path::new(&path_filename).is_file() == true{
                    fetch_files.push(path_filename);
                }

                }

            return fetch_files;

        }

    }
