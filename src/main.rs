use walkdir::{WalkDir,DirEntry};
use std::fs::File;
use std::io::prelude::*;
use scraper::{Html,Selector};
use regex;

#[derive(Debug)]
#[allow(dead_code)]
enum Citations{
    Articles(String),
    Cases(String),
    Other(String),
    None
}


#[allow(dead_code)]
struct Divisions{
    cases: Vec<Jurisprudence>,

}

#[allow(dead_code)]
struct Persons {
    title: String,
    status: String,
}

#[allow(dead_code)]
struct Jurisprudence{
    id: String,
    title: String,
    citations: Vec<Citations>,
    body: String,
    date: String,
    // complainant: String,
    // respondent: String,
    // division: String,
    // Judge: String,
}

fn get_all_files() -> Vec<DirEntry> {
    
    let mut files = Vec::new();

    for entry in WalkDir::new("juris") {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file(){
            files.push(entry)
        }
    }
    println!("file length: {}",files.len());
    return files;
}
fn parse_file(input: &DirEntry) -> Jurisprudence{
   
    // Opens the file.    println!("File name{}",file.path().display());
    let path = input.path(); 
    let display=path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }

    // Parse html
    let html = Html::parse_document(&s);
    
    // let h2_selector = Selector::parse("h2").unwrap();
    let h3_selector = Selector::parse("h3")
                                .expect("Error Found here");
    // let h2_element = &mut html.select(&h2_selector);
    let h3_element = &mut html.select(&h3_selector)
                                .next()
                                .expect("selecting the title failed");
    
    let main_selector = Selector::parse("div")
                                .expect("error while parsing Div");
    let main_element = &mut html.select(&main_selector)
                                .next()
                                .expect("Selecting Div Main Failed");
    let juris = Jurisprudence {
        id:         String::from(input.path().file_name().unwrap().to_string_lossy().into_owned()),
        title:      String::from(h3_element.inner_html()),
        body:       String::from(main_element.inner_html()) ,
        citations:  vec![
                        Citations::Other("Test Citations".to_string()),
                        Citations::Articles("Test".to_string())
                    ] ,
        date:       String::from("Dec 12, 2000"),
        };
    return juris;


    //
    
    //
    // 

}

//fn create_json_object() {
//
//}

fn main() {
    let files = get_all_files();
    // let mut counter = 0;
    // for file in files {
    //     let _title = parse_file(&file);
    //     counter+=1;
    //     if counter % 500 == 0{
    //         println!("{}",counter);
    //     }
    // }
    let juris = parse_file(&files[1]);
    println!("Title:{} \nid:{}\ncitations:{:?}\n",juris.title, juris.id, juris.citations, );
    let split = juris.title.split("VS.");
    for piece in split{
        println!("{}",piece);
    }
}
