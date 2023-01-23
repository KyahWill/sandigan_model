use walkdir::DirEntry;
use std::fs::{File, copy};
use std::io::prelude::*;
// use std::vec;
use scraper::{Html,Selector};
use regex::{Regex};
use csv::{Reader};
use chrono::Month;

// #[derive(Clone)]
// struct Jargon {
//     title: String,
//     id: usize,
//     definition: String,
// }
pub struct LegalTerm {
    pub id: String,
    pub title: String,
}

pub struct Jurisprudence{

    pub id: String,
    pub title: String,
    pub url: String,
    pub gr_number: Vec<String>,
    pub year: String,
    pub month: String,
    pub day: String,
    pub division: String,
    pub case_code: String,
    pub citations: Vec<String>,
}
pub fn path_to_html_string(input_path: DirEntry) -> Html {
    let mut file = match File::open(input_path.path()) {
        Err(why) => panic!("couldn't open {}: {}", input_path.path().display(), why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read: {}", why),
        Ok(_) => {},
    }

    // Parse html
    return Html::parse_document(&s);
    

}

pub fn get_all_legal_words() -> Vec<LegalTerm>{
    let mut juris_rdr: Reader<File> = Reader::from_path("jargon.csv").unwrap();
    let mut jargons:Vec<LegalTerm> = vec![];
    for record in juris_rdr.records().skip(1) {
        let record_data = record.unwrap();
        jargons.push(LegalTerm {
            id: record_data
                .get(1)
                .unwrap().to_string(),
            title: record_data.get(0).unwrap().to_string(),
        });
    }
    return jargons;
}

pub fn get_juris_details(html: Html, html_string: String,input_path: DirEntry, id: usize) -> Jurisprudence {

    // h2 selector selects the division
    let h2_selector = Selector::parse("h2")
                                .expect("Error found here");
    let h2_element = &mut html.select(&h2_selector)
                                .next()
                                .expect("Selecting h2 failed");
    let division = String::from(h2_element.inner_html());
    
    let juris_code_regex = Regex::new("<body>[\\n\\r\\s]+*.*[\\n\\r\\s]+<").unwrap();
    let juris_capture = juris_code_regex.captures(html_string.as_str()).unwrap_or(juris_code_regex.captures("<body> unknown <").unwrap());
    let juris_code =juris_capture.get(0).map_or("".to_string(), |m| m.as_str().to_string());
    let juris_result = juris_code.replace("<body>","")
                            .replace("<center>", "")
                            .replace("<","") 
                            .trim()
                            .replace("g. ", "g.")
                            .replace("a. ", "a.")
                            .replace("","")
                            ;
    //selects the title of the jurisprudence
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
                            "").replace("&amp;","&");

    //h4 selector selects GR Number.
    let h4_selector = Selector::parse("h4")
                            .expect("Error Found here");
    let h4_element = html.select(&h4_selector)
                                .next()
                                .expect("selecting the title failed");
    let date_regex = Regex::new("\\w+\\s\\d{2},\\s\\d{4}").unwrap();
    let h4_binding =  h4_element.clone().inner_html().replace("[ ", "").replace('\n', "").replace('\t',"");
    let h4_text = h4_binding.trim();

    let h4_cap = date_regex.captures(&h4_text).unwrap_or(date_regex.captures("January 01, 1800").unwrap());
    let date = h4_cap.get(0).map_or("".to_string(), |m| m.as_str().to_string());
    
    let mut gr_number = date_regex.replace_all(h4_text, "")
                        .to_lowercase()
                        .replace("&amp;", "&")
                        .replace("g. ", "g.")
                        .replace("a. ", "a.")
                        .replace("nos.", "no.")
                        .replace("  ", " ")
                        .trim()
                        .to_string();

    if gr_number.ends_with(",") {
        gr_number.pop().unwrap();
    }

    let year = date.split(",").nth(1).unwrap().to_string().trim().to_string();
    let month = date.split(",")
                        .nth(0).unwrap()
                        .split(" ")
                        .nth(0).unwrap()
                        .parse::<Month>().unwrap()
                        .number_from_month()
                        .to_string();
    let day = date.split(",").nth(0).unwrap().split(" ").nth(1).unwrap().to_string();

    // Create citations within the document. God please work
    let mut citations:Vec<String> = Vec::new();
    let citation_regex = Regex::new(r"</sup>(?P<title>[^']+)<br><br>").unwrap(); 
    let citation_capture = citation_regex.captures_iter(html_string.as_str());
    for citation in citation_capture{
        citations = citation.name("title").unwrap().as_str()
                .split("<sup style=\"color: rgb(255, 0, 0);\">")
                .collect::<Vec<&str>>()
                .iter().map(|&x| x.into()).collect();

        
    }

    let output = Jurisprudence{

        id: String::from("juris_")+ id.to_string().as_str(),
        title: result.to_string(),
        url: input_path.path()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
        gr_number: gr_number,
        year: year,
        month: month,
        day: day,
        division: division,
        case_code: juris_result,
        citations: citations,

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

#[allow(dead_code)]
pub fn move_all_files(files: Vec<DirEntry>) {
    let mut juris_id = 1;
    for file in files { 
        let path = file.path();
        let file_name = path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap();
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