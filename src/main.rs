use commands::JurisCitations;
use csv::{Reader, Writer};
use mongodb::{bson::doc, options::ClientOptions, Client};
use regex::Regex;
use scraper::Selector;
use std::fs::File;
use std::{collections::HashMap, io::Write};
use walkdir::{DirEntry, WalkDir};

mod commands;

fn get_all_files() -> Vec<DirEntry> {
    let mut files: Vec<DirEntry> = Vec::new();

    for entry in WalkDir::new("juris") {
        let entry = entry.unwrap();
        if entry.metadata().unwrap().is_file() {
            files.push(entry)
        }
    }
    println!("file length: {}", files.len());
    return files;
}

struct KnowledgeGraph {
    head: usize,
    tail: usize,
    relation: String,
}

#[allow(dead_code)]
fn upload_to_mongodb(juris: commands::Jurisprudence, raw_text: &str) {
    println!("{:?}, {}", juris, raw_text);
}

#[allow(dead_code)]
fn parse_all_files(
    files: Vec<DirEntry>,
    file_length: usize,
) -> (Vec<KnowledgeGraph>, Vec<commands::Jurisprudence>) {
    // File Citations
    let mut output: Vec<KnowledgeGraph> = Vec::new();
    let mut output_juris: Vec<commands::Jurisprudence> = Vec::new();
    let mut citation_wtr = Writer::from_path("citation.csv").unwrap();
    citation_wtr
        .write_record(&["juris_id", "case_citations"])
        .unwrap();

    // // File to Legal Term Connection
    // let mut legal_term_wtr = Writer::from_path("knowledge_graph.csv").unwrap();
    // legal_term_wtr.write_record(&["juris_id","legal_term_id","label"]).unwrap();

    let legal_terms = commands::get_all_legal_words();

    let gr_regex = Regex::new(r"(g.?\s?r.?\s+)(nos?.?\s+)([1-9][0-9]{0,6})").unwrap();
    let juris_code_regex =
        Regex::new(r"([1-9][0-9]{0,4})(\s[a-zA-Z]+\.\s+)([1-9][0-9]{0,4})").unwrap();

    for (juris_id, file) in files.iter().enumerate() {
        let html_input = commands::path_to_html_string(file.to_owned());
        let html_selector = Selector::parse("html").unwrap();
        let html_element = &mut html_input
            .select(&html_selector)
            .next()
            .expect("Selecting the html element failed");
        let html_string = html_element.inner_html().to_lowercase();

        let borrowed_html_string = html_string.as_str();

        let juris: commands::Jurisprudence =
            commands::get_juris_details(html_input, html_string.clone(), juris_id);

        //writes down the list of jurisprudence in excel
        for citation in juris.citations.clone() {
            match gr_regex.captures(citation.clone().as_str()) {
                Some(capture) => {
                    let gr_string_regex: Regex = Regex::new(r"(g.?\s?r.?\s+)(nos?.?\s+)").unwrap();
                    let string = gr_string_regex
                        .replace_all(capture.get(0).unwrap().as_str(), "g.r. no. ")
                        .replace("&amp;", "&");
                    // println!("GR Code: {}",string);
                    citation_wtr
                        .write_record(&[juris.id.clone().to_string(), string])
                        .unwrap();
                }
                None => {
                    match juris_code_regex.captures(citation.clone().as_str()) {
                        Some(capture) => {
                            let string = capture
                                .get(0)
                                .unwrap()
                                .as_str()
                                .replace("a. ", "a.")
                                .replace("nos.", "no.")
                                .replace("  ", " ");
                            // println!("Juris: {}",string);
                            citation_wtr
                                .write_record(&[juris.id.to_string(), string])
                                .unwrap();
                        }
                        None => {}
                    }
                }
            }
        }
        output_juris.push(juris);
        for term in &legal_terms {
            // println!("{}", term.title);
            if borrowed_html_string.contains((term.title.clone() + " ").as_str())
                || borrowed_html_string.contains((term.title.clone() + ".").as_str())
            {
                output.push(KnowledgeGraph {
                    head: juris_id,
                    tail: term.id + file_length,
                    relation: String::from("MENTIONS"),
                });
                // legal_term_wtr.write_record(&[juris.id.clone(), term.id.clone().to_string()]).unwrap();
            }
        }

        if juris_id % 500 == 0 {
            println!("{}", juris_id)
        }
    }
    return (output, output_juris);
}

// fn parse_all_citations() {
// }

#[allow(dead_code)]
fn binary_search(items: &Vec<&String>, find: &String) -> Option<usize> {
    let length = items.len();
    let mut half = length / 2;
    let mut hind = length - 1;
    let mut lind = 0;
    let mut current = items[half];
    while lind <= hind {
        if current.contains(find) {
            return Some(half);
        } else if current < find {
            lind = half + 1
        } else if current > find {
            hind = half - 1;
        }
        half = (hind + lind) / 2;
        current = items[half];
    }
    return None;
}

fn linear_search(items: &Vec<&String>, find: &String) -> Option<usize> {
    let length = items.len();
    for (index, item) in items.iter().enumerate() {
        if item.contains(find){
            return Some(index);
        }
    }
    return None;
}

#[allow(dead_code)]
fn convert_citations_to_juris(juris_list: Vec<commands::Jurisprudence>) -> Vec<KnowledgeGraph> {
    let mut output: Vec<KnowledgeGraph> = Vec::new();

    let mut juris_hash_map: HashMap<String, String> = HashMap::new();
    // open citation
    let mut citation_rdr: Reader<File> = Reader::from_path("citation.csv").unwrap();

    let mut juris_to_none_wtr: Writer<File> = Writer::from_path("juris_to_none.csv").unwrap();
    juris_to_none_wtr
        .write_record(&["source", "citation"])
        .unwrap();

    // for every juris.row
    for juris in juris_list {
        // if key doesn't exist, create hashmap item containing {key = GR number, value = juris_id}
        juris_hash_map
            .entry(juris.gr_number)
            .or_insert(juris.id.to_string());
        // if key doesn't exist, Create hashmap item containing {key = File_code, value = juris_id}
        juris_hash_map
            .entry(juris.case_code)
            .or_insert(juris.id.to_string());
    }

    // create array of hashmap keys.
    let mut juris_hash_keys = Vec::from_iter(juris_hash_map.keys());
    juris_hash_keys.sort();
    // for every citatation.row
    let mut counter = 0;
    let citation_records = citation_rdr.records().skip(1);
    for record in citation_records {
        let record_data = record.unwrap();
        let citation = record_data.get(1).unwrap().to_string();
        let juris_match = binary_search(&juris_hash_keys, &citation);
        //I have to implement this myself.
        match juris_match {
            Some(index) => {
                let input = &juris_hash_map[juris_hash_keys[index]];

                output.push(KnowledgeGraph {
                    head: record_data.get(0).unwrap().parse::<usize>().unwrap(),
                    tail: input.parse::<usize>().unwrap(),
                    relation: String::from("CITES"),
                });
            }
            None => {
                juris_to_none_wtr
                    .write_record(&[record_data.get(0).unwrap(), record_data.get(1).unwrap()])
                    .unwrap();
            }
        }
        if counter % 500 == 0 {
            println!("{}", counter)
        }
        counter += 1;
    }

    return output;
    // juris to juris write juris.id jurismatch.id
    // */

    /*
        input = list of strings,
    LOAD CSV WITH HEADERS from "https://raw.githubusercontent.com/KyahWill/sandigan_model/main/juris_to_juris.csv" AS line
    WITH  line
    MATCH(src:Juris),
    (citation:Juris)
    where src.unique_id = line.source AND src.unique_id = line.juris_id
    CREATE(src)-[:CITES]->(citation)
        */
}
// #[tokio::main]
// async
#[allow(dead_code)]
fn create_knowledge_graph(
    juris_to_juris: Vec<KnowledgeGraph>,
    juris_to_legal_term: Vec<KnowledgeGraph>,
) {
    let mut knowledge_graph_wtr = Writer::from_path("citation.csv").unwrap();
    knowledge_graph_wtr
        .write_record(&["juris_id", "case_citations", "relationship"])
        .unwrap();
    for entry in juris_to_juris {
        knowledge_graph_wtr
            .write_record(&[
                entry.head.to_string(),
                entry.tail.to_string(),
                entry.relation,
            ])
            .unwrap();
    }
    for entry in juris_to_legal_term {
        knowledge_graph_wtr
            .write_record(&[
                entry.head.to_string(),
                entry.tail.to_string(),
                entry.relation,
            ])
            .unwrap();
    }
}

#[allow(dead_code)]
fn create_identity_entity_matrix(length: usize) {
    let mut identity_entity_wtr = std::fs::File::create("item_id2entity_id.txt").unwrap();

    for i in 0..length {
        let new_string = format!("{}\t{}\n", i.to_string().as_str(), i.to_string().as_str());
        identity_entity_wtr
            .write_all(new_string.as_bytes())
            .unwrap();
    }
}

fn parse_data_set(files: Vec<DirEntry>, file_length: usize) -> Vec<commands::JurisCitations> {
    let mut output: Vec<commands::JurisCitations> = Vec::new();

    for (juris_id, file) in files.iter().enumerate() {
        let html_input = commands::path_to_html_string(file.to_owned());
        let html_selector = Selector::parse("html").unwrap();
        let html_element = &mut html_input
            .select(&html_selector)
            .next()
            .expect("Selecting the html element failed");
        let html_string = html_element.inner_html().to_lowercase();

        let juris: commands::JurisCitations =
            commands::get_juris_case_code(html_input, html_string.clone(), juris_id);

        output.push(juris);
        //writes down the list of jurisprudence in excel

        if juris_id % 500 == 0 {
            println!("{}", juris_id)
        }
    }

    return output;
}

fn create_test_data() {

    let mut juris_hash_map: HashMap<String, String> = HashMap::new();

    let mut juris_rdr: Reader<File> = Reader::from_path("juris.csv").unwrap();
    // open citation
    let mut citation_rdr: Reader<File> = Reader::from_path("test_data_google.csv").unwrap();

    let mut citation_wtr: Writer<File> = Writer::from_path("training_data.csv").unwrap();
    let mut juris_to_none_wtr: Writer<File> = Writer::from_path("test_data_null.csv").unwrap();
    juris_to_none_wtr
        .write_record(&["source", "citation"])
        .unwrap();

    //create hashmap
    // for every juris.row
  
    for record in juris_rdr.records().skip(1) {
        let record_data = record.unwrap();
        // if key doesn't exist, create hashmap item containing {key = GR number, value = juris_id}
        juris_hash_map.entry(record_data.get(6).unwrap().to_string())
                    .or_insert(record_data.get(0).unwrap().to_string());
        // if key doesn't exist, Create hashmap item containing {key = File_code, value = juris_id}
        juris_hash_map.entry(record_data.get(8).unwrap().to_string())
                    .or_insert(record_data.get(0).unwrap().to_string());
        }


    // create array of hashmap keys.
    let mut juris_hash_keys = Vec::from_iter(juris_hash_map.keys());
    juris_hash_keys.sort();
    
    // println!("{}",juris_hash_keys.len());
    // let index =  linear_search(&juris_hash_keys, &"g.r. no. 174912".to_string()).unwrap();
    // let input = &juris_hash_map[juris_hash_keys[index]];
    // println!("{}", input);


    // for every citatation.row
    let mut counter = 0;
    let citation_records = citation_rdr.records().skip(1);
    for record in citation_records {
        let record_data = record.unwrap();
        let case: String = record_data.get(0).unwrap().to_string();
        let citation = record_data.get(2).unwrap().to_string();
        let case_code = record_data.get(3).unwrap().to_string();
        
        println!("{}, {}, {}", case, citation, case_code);
        let juris_match = linear_search(&juris_hash_keys, &citation);
        //I have to implement this myself.
        if &citation != "N/A" {
            match juris_match {
                Some(index) => {
                    let input = &juris_hash_map[juris_hash_keys[index]];
                    citation_wtr.write_record(&[
                        case.clone(),
                        input.to_string(),
                    ]).unwrap();
                }
                None => {
                    juris_to_none_wtr
                    .write_record(&[record_data.get(0).unwrap(), record_data.get(2).unwrap()])
                    .unwrap();
                }
            }
        }
        if &case_code != "N/A" {
            match linear_search(&juris_hash_keys, &case_code) {
                Some(index) => {
                    let input = &juris_hash_map[juris_hash_keys[index]];
                    citation_wtr.write_record(&[
                        case,
                        input.to_string(),
                    ]).unwrap();
                    },
                None => {
                    juris_to_none_wtr
                    .write_record(&[record_data.get(0).unwrap(), record_data.get(3).unwrap()])
                    .unwrap();
                }
            }
        }
        if counter % 500 == 0 {
            println!("{}", counter)
        }
        counter += 1;
    }

}

fn main() {
    // let client_options =
    //     ClientOptions::parse("mongodb+srv://<username>:<password>@<cluster-url>/test?w=majority")
    //     .await?;

    //  // Get a handle to the cluster
    // let client = Client::with_options(client_options)?;
    // // Ping the server to see if you can connect to the cluster
    // client
    //     .database("admin")
    //     .run_command(doc! {"ping": 1}, None)
    //     .await?;
    // println!("Connected successfully.");

    // for db_name in client.list_database_names(None, None).await? {
    //     println!("{}", db_name);
    // }
    // Ok(())
    // get all files

    // let files: Vec<DirEntry> = get_all_files();
    // let (juris_to_legal_term, juris) = parse_all_files(files.clone(), files.len());
    // let juris_case_codes: Vec<commands::JurisCitations> = parse_data_set(files.clone(), files.len());
    create_test_data();    

    // let juris_to_juris: Vec<KnowledgeGraph> = convert_citations_to_juris(juris);
    // create_knowledge_graph(juris_to_juris, juris_to_legal_term)
    // move_all_files(files)
    // create_identity_entity_matrix(files.len())
}
