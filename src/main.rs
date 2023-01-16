use walkdir::{WalkDir,DirEntry};
use csv::{Writer};
mod commands;

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
#[allow(dead_code)]

fn parse_all_files( files: Vec<DirEntry>) {
    let mut juris_wtr = Writer::from_path("juris.csv").unwrap() ;
    juris_wtr.write_record(&["identifier","title","file_url","year","month","day","gr Number", "division", "case_code"]).unwrap();
    
    let mut citation_wtr = Writer::from_path("citation.csv").unwrap();
    citation_wtr.write_record(&["juris_id","case_citations"]).unwrap();

    for (juris_id ,file) in files.iter().enumerate()  { 

        //
        let html_input = commands::path_to_html_string(file.to_owned());
        let juris = commands::get_juris_details(html_input, file.to_owned(),juris_id);
        // println!("{}, {}", file.path().display().to_string(), juris.citations.len());       
        //writes down the list of jurisprudence in excel
        juris_wtr.write_record(&[juris.id,juris.title, juris.url, juris.year, juris.month, juris.day, juris.gr_number, juris.division, juris.case_code]).unwrap();
        
        // Write down the list of citations.
        for citation in juris.citations {
            citation_wtr.write_record(&[juris_id.to_string(), citation]).unwrap();
        }

        if juris_id %500 == 0 {
            println!("{}",juris_id)
        }
    }
}

// fn parse_all_citations() {


// }

fn main() {
    let files: Vec<DirEntry> = get_all_files();
    parse_all_files(files);

    // move_all_files(files)
}  
