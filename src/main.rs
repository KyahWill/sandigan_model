use walkdir::{WalkDir,DirEntry};
use csv::{Reader,Writer};
use scraper::{Selector};
use regex::{Regex};
use std::fs::File;
use std::collections::HashMap;
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
    // File Description
    let mut juris_wtr = Writer::from_path("juris.csv").unwrap() ;
    juris_wtr.write_record(&["identifier","title","file_url","year","month","day","gr Number", "division", "case_code"]).unwrap();
    
    // File Citations
    let mut citation_wtr = Writer::from_path("citation.csv").unwrap();
    citation_wtr.write_record(&["juris_id","case_citations"]).unwrap();

    // File to Legal Term Connection
    let mut legal_term_wtr = Writer::from_path("juris_to_legalterm.csv").unwrap();
    legal_term_wtr.write_record(&["juris_id","legal_term_id"]).unwrap();

    let legal_terms = commands::get_all_legal_words();


    let gr_regex = Regex::new(r"(g.?\s?r.?\s+)(nos?.?\s+)([1-9][0-9]{0,6})").unwrap();
    let juris_code_regex = Regex::new(r"([1-9][0-9]{0,4})(\s[a-zA-Z]+\.\s+)([1-9][0-9]{0,4})").unwrap();
    for (juris_id ,file) in files.iter().enumerate()  { 

        //

        let html_input = commands::path_to_html_string(file.to_owned());

        let html_selector = Selector::parse("html").unwrap();
        let html_element = &mut html_input.select(&html_selector).next().expect("Selecting the html element failed");
        let html_string = html_element.inner_html().to_lowercase().replace("&amp;","&").replace("&nbsp;", "");
        let borrowed_html_string = html_string.as_str();
    
        let juris = commands::get_juris_details(html_input, html_string.clone(),file.to_owned(),juris_id);
        // println!("{}, {}", file.path().display().to_string(), juris.citations.len());       
        //writes down the list of jurisprudence in excel
        juris_wtr.write_record(&[juris.id.clone(),juris.title, juris.url, juris.year, juris.month, juris.day, juris.gr_number, juris.division, juris.case_code]).unwrap();
        
        // Write down the list of citations.
        for citation in juris.citations {
            match gr_regex.captures(citation.clone().as_str()){
                Some(capture) => {
                    let string = capture.get(0).unwrap().as_str()
                            .replace("&amp;", "&")
                            .replace("g. ", "g.")
                            .replace("  ", " ")
                            .replace("nos.", "no.");
                    // println!("GR Code: {}",string);
                    citation_wtr.write_record(&[juris_id.to_string(), string]).unwrap();
                }
                None => {
                    match juris_code_regex.captures(citation.clone().as_str()){
                        Some(capture) => {
                            let string = capture.get(0).unwrap().as_str()
                                .replace("a. ", "a.")
                                .replace("nos.", "no.")
                                .replace("  ", " ");
                            // println!("Juris: {}",string);
                            citation_wtr.write_record(&[juris_id.to_string(), string]).unwrap();
                        }
                        None => {
                        }
                    }
                }
            }
        }

        for term in &legal_terms {
            // println!("{}", term.title);
            if borrowed_html_string.contains(&term.title){
                legal_term_wtr.write_record(&[juris.id.clone(), term.id.clone().to_string()]).unwrap();
            }
        }
        
        if juris_id %500 == 0 {
            println!("{}",juris_id)
        }
    }
}

// fn parse_all_citations() {
// }
fn binary_search(items: &Vec<&String>, find: &String) -> Option<usize> {
    let length = items.len();
    let mut half = length / 2;
    let mut hind = length - 1;
    let mut lind = 0;
    let mut current = items[half];
    while lind <= hind {
        if current.contains(find){
            return Some(half);
        }
        else if current < find {
            lind = half + 1
        }
        else if current > find {
            hind = half - 1;
        }
        half = (hind + lind ) / 2;
        current = items[half];
    }
    return None;
}

fn convert_citations_to_juris() {
    let mut juris_hash_map: HashMap<String, usize> = HashMap::new();
    // open juris
    let mut juris_rdr: Reader<File> = Reader::from_path("juris.csv").unwrap();
    // open citation
    let mut citation_rdr: Reader<File> = Reader::from_path("citation.csv").unwrap();


    let mut juris_to_juris_wtr: Writer<File> = Writer::from_path("juris_to_juris.csv").unwrap();
    juris_to_juris_wtr.write_record(&["source", "juris_id"]).unwrap();
    let mut juris_to_none_wtr: Writer<File> = Writer::from_path("juris_to_none.csv").unwrap();
    juris_to_none_wtr.write_record(&["source","citation"]).unwrap();


    // for every juris.row    
    for record in juris_rdr.records().skip(1) {
        let record_data = record.unwrap();
        // if key doesn't exist, create hashmap item containing {key = GR number, value = juris_id}
        juris_hash_map.entry(record_data.get(6).unwrap().to_string())
                    .or_insert(record_data.get(0).unwrap().parse::<usize>().unwrap());
        // if key doesn't exist, Create hashmap item containing {key = File_code, value = juris_id}
        juris_hash_map.entry(record_data.get(8).unwrap().to_string())
                    .or_insert(record_data.get(0).unwrap().parse::<usize>().unwrap());
    }


    // create array of hashmap keys.
    let mut juris_hash_keys = Vec::from_iter(juris_hash_map.keys());
    juris_hash_keys.sort();
    // for every citatation.row
    let mut counter = 0;
    let citation_records = citation_rdr.records().skip(1);
    for record in  citation_records{
        let record_data = record.unwrap();
        let citation = record_data.get(1).unwrap().to_string();
        let juris_match = binary_search(&juris_hash_keys, &citation);
         //I have to implement this myself.
         match juris_match{
            Some(index) => {

                let input = juris_hash_map[juris_hash_keys[index]];

                juris_to_juris_wtr.write_record(&[record_data.get(0).unwrap().to_string(),input.to_string()]).unwrap();
            }
            None => {
                juris_to_none_wtr.write_record(&[record_data.get(0).unwrap(),record_data.get(1).unwrap()]).unwrap();
            }
         }
        if counter %500 == 0 {
            println!("{}",counter)
        }
        counter += 1;
    }
    
    // juris to juris write juris.id jurismatch.id
    // */

   

    
    /* 
    input = list of strings,

    */
    

}

fn main() {
    // let files: Vec<DirEntry> = get_all_files();
    // parse_all_files(files);
    convert_citations_to_juris();

    // move_all_files(files)
}  
