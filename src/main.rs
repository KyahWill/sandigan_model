use walkdir::{WalkDir,DirEntry};
use csv::{Writer};
mod commands;
use std::fs::copy;
fn get_all_files() -> Vec<DirEntry> {
    
    let mut files: Vec<DirEntry> = Vec::new();

    for entry in WalkDir::new("juris") {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file(){
            files.push(entry)
        }
    }
    println!("file length: {}",files.len());
    return files;
}

fn move_all_files(files: Vec<DirEntry>) {
    let mut juris_id = 1;
    for file in files { 
        let path = file.path();
        let file_name = path
        .file_name().unwrap().to_str().unwrap();
        let destination = format!("temp_target/{}",file_name);
                                                        // .unwrap()
                                      
                                                        // .to_str().to_string());
        
        copy(path,destination).unwrap();
        // let juris = commands::get_juris_details(file, juris_id);
        juris_id+=1;
        if juris_id %500 == 0 {
            println!("{}",juris_id)
        }
    }
}
#[allow(dead_code)]
fn parse_all_files( files: Vec<DirEntry>) {
    let mut juris_wtr = Writer::from_path("juris.csv").unwrap() ;
    juris_wtr.write_record(&["identifier","title","file_url","year","month","day","gr Number"]).unwrap();
    let mut juris_id = 1;
    for file in files { 
        let juris = commands::get_juris_details(file, juris_id);
        juris_id+=1;
        juris_wtr.write_record(&[juris.id,juris.title, juris.url, juris.year, juris.month, juris.day, juris.gr_number]).unwrap();
        if juris_id %500 == 0 {
            println!("{}",juris_id)
        }
    }

}

// fn parse_all_citations() {


// }

fn main() {
    let files: Vec<DirEntry> = get_all_files();
    // parse_all_files(files);
    move_all_files(files)
}  
