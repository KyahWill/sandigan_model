use walkdir::{WalkDir,DirEntry};
use std::fs::File;
use std::io::prelude::*;
use scraper::{Html,Selector};
use regex::Regex;
use csv::{Writer, Reader};


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
struct PseudoJuris {
    title: String,
    url: String,
    jargons: Vec<usize>,
}
#[derive(Clone)]
struct Jargon {
    title: String,
    id: usize,
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
fn get_juris_details(input: &DirEntry, jargons: Vec<Jargon>) -> PseudoJuris {
   
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
    
    // h2 selector selects the division
    // let h2_selector = Selector::parse("h2")
                                // .expect("Error found here");
    // let h2_element = &mut html.select(&h2_selector)
                                // .next()
                                // .expect("Selecting h2 failed");
    // let _h2_text = String::from(h2_element.inner_html());
    // println!("H2_element = {}", h2_text );
    
    
    //h3 selector selects the plaintiff/apellant or their role, can be separated via commas.
    let h3_selector = Selector::parse("h3")
                                .expect("Error Found here");
    let h3_element = &mut html.select(&h3_selector)
                                .next()
                                .expect("selecting the title failed");
    let h3_text =  String::from(h3_element.inner_html());
    let h3_split = h3_text.split("<br><br>")
                            .nth(0)
                            .unwrap();
    let title_regex = Regex::new("<sup\\s*.*>\\s*.*</sup>").unwrap();
    let italic_regex = Regex::new("<i\\s*.*>").unwrap();
    let italic_end_regex = Regex::new("</i>").unwrap();
    let result = italic_end_regex
                    .replace(italic_regex
                            .replace(title_regex
                                     .replace(h3_split,"").as_ref(),
                                    "").as_ref(),
                            "").to_string();
    
    //Create a list of legal jargon
    let mut juris_jargons = vec![];
    for jargon in jargons {
        if s.contains(&jargon.title){
            juris_jargons.push(jargon.id)
        }
    }



    let output = PseudoJuris{
        title: result.to_string(),
        url: input.path().display().to_string(),
        jargons: juris_jargons,
    }; 





    //println!("{:?}", result); 

    // for party in h3_split.split("VS."){

     //   println!("{}",party.split(",").nth(0).unwrap());
     //   println!("{}",party.split(",").nth(1).unwrap());
    //}
    //h3_split = h3_split.split("VS.")
    //                    .nth(0)
    //                    .unwrap();
    //println!("Parties Involved:");
    //for word in h3_split.split(",") {
    //    println!("{}",word.trim());
    //}

    //h4 selector selects GR Number.
    // let h4_selector = Selector::parse("h4")
    //                            .expect("Error Found here");
    // let h4_element = &mut html.select(&h4_selector)
    //                            .next()
    //                            .expect("selecting the title failed");
    //let citation_vector = std::Vec<std::String>;
    // let identifier =  String::from(h4_element.inner_html());


    //find where the first <hr> is and then select all the text below it. 
    
    //if s.contains("<hr") {
    //    println!("Found Horizontal Line");
    //}
    //let main_selector = Selector::parse("div")
    //                            .expect("error while parsing Div");
    //let main_element = &mut html.select(&main_selector)
    //                            .next()
    //                            .expect("Selecting Div Main Failed");
    //let juris = Jurisprudence {
    //    id:         String::from(input.path().file_name().unwrap().to_string_lossy().into_owned()),
    //    title:      String::from(h3_element.inner_html()),
    //    body:       String::from(main_element.inner_html()) ,
    //    citations:  vec![
    //                    Citations::Other("Test Citations".to_string()),
    //                    Citations::Articles("Test".to_string())
    //                ] ,
    //   date:       String::from("Dec 12, 2000"),
    //    };
    //return juris;
    //
    //
    //
    // let binding = identifier.trim();
    return output;
}

//fn create_json_object() {
//
//}

fn main() {
    let mut juris_rdr = Reader::from_path("jargon.csv").unwrap();
    let mut jargons = vec![];
    for record in juris_rdr.records().skip(1) {
        let record_data = record.unwrap();
        jargons.push(Jargon {
            id: record_data
                .get(1)
                .unwrap().to_string().parse().unwrap(),
                
            title: record_data.get(0).unwrap().to_string(),
        });
    }


    let mut juris_wtr = Writer::from_path("juris.csv").unwrap() ;
    let mut rel_wtr = Writer::from_path("juristojargon.csv").unwrap();
    
    let files = get_all_files();
    

    juris_wtr.write_record(&["title","identifier","file_url"]).unwrap();
    rel_wtr.write_record(&["juris_id","jargon"]).unwrap();
    let mut counter = 0;
    
    for file in files {
        let juris= get_juris_details(&file, jargons.clone());
        juris_wtr.write_record(&[juris.title, counter.to_string(),juris.url]).unwrap();
        for jargon in juris.jargons {
            rel_wtr.write_record(&[counter.to_string(), jargon.to_string()]).unwrap();
        }
        counter+=1;
        if counter % 500 == 0{
            println!("{}",counter);
        }
    }
    juris_wtr.flush().unwrap();
    //let _juris = parse_file(&files[1]);
    
    //println!("Title:{} \nid:{}\ncitations:{:?}\n",juris.title, juris.id, juris.citations, );
    // let split = juris.title.split("VS.");
    //for piece in split{
    //    println!("{}",piece);
    //
}
