use std::fs;
pub mod cfgtools;

fn main() {
    let data = fs::read_to_string("./input.json")
        .expect("Unable to read file");
    let json: serde_json::Value = serde_json::from_str(&data).expect("Invalid");
    
    let variables: Vec<&str> = json["variables"].as_array().expect("Variables not an array")
    .iter().map(|x| x.as_str().expect("Your var name is invalid") ).collect();
    let alphabet: Vec<&str> = json["alphabet"].as_array().expect("Alphabet not an array")
    .iter().map(|x| 
        x.as_str().expect("Cannot convert alphabet to string"))
        .collect();     
    let productions: Vec<Vec<&str>> = json["productions"].as_array().expect("Productions array invalid").iter()
    .map(|x| {
        x.as_array().expect("Production invalid").iter()
        .map(|y| y.as_str().expect("Production invalid"))
        .collect()
    }
    ).collect();
    // let productions: Vec<Vec<&str>> = 
    // &json["productions"].as_array().expect("Productions array invalid").iter()
    // .map(|x| {
    //     x.as_array().expect("Production invalid").iter()
    //     .map(|y| {
    //         y.as_str().expect("Production character invalid")
    //     }).collect::<Vec<&str>>()
    // })
    // .collect();
    // println!("Variables are {:?}", &variables);
    // println!("Alphabet are {:?}", &alphabet);
    // println!("Productions are {:?}", &productions);
    let start_variable = json["start_variable"].as_str().expect("Start variable invalid");
    cfgtools::cfgtools::create_parsing_table(
        &alphabet, 
        &variables, 
        &productions, start_variable)
}
