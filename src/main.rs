use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use chrono::Utc;


fn main() {
    let file_names = vec!["empty_sample_data.csv", "sample_data.csv", "sample_data_huge.csv", "sample_data_large.csv"];
    file_names.iter().for_each(|file| {
        test(file.to_string())
    })
}

fn read_in_file(file_name: String) -> Vec<Data> {
    BufReader::new(File::open(file_name).unwrap())
        .lines()
        .skip(1)
        .map(|x| {
            let split: Vec<String> = x
                .unwrap()
                .split(",")
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            Data {
                tan: split.get(0).unwrap().to_string(),
                not: split.get(1).unwrap().to_string(),
                city: split.get(2).unwrap().to_string(),
                province: split.get(3).unwrap().to_string(),
                pi: split.get(4).unwrap().to_string(),
                ri: split.get(5).unwrap().to_string(),
            }
        })
        .collect()
}

fn test(file_name: String) {

    let file = file_name.clone();
    let mut data: Vec<Data> = read_in_file(file_name);
    println!("~~~~~~~~~ {:?} ({:?})~~~~~~~~~", file, data.len());
    
    data = add_to_middle(
        data,
        Data {
            tan: "my Testtan".to_string(),
            not: "my Testnot".to_string(),
            city: "my Testcity".to_string(),
            province: "my Testprovince".to_string(),
            pi: "my Testpi".to_string(),
            ri: "my Testri".to_string(),
        },
    );

    data = add_to_end(
        data,
        Data {
            tan: "my Testtan".to_string(),
            not: "my Testnot".to_string(),
            city: "my Testcity".to_string(),
            province: "my Testprovince".to_string(),
            pi: "my Testpi".to_string(),
            ri: "my Testri".to_string(),
        },
    );

    data = remove_end(data);

    remove_middle(data);
    println!("\n")
}

fn add_to_middle(mut data: Vec<Data>, new_data: Data) -> Vec<Data> {
    let start_time = Utc::now().time();
    data.insert(data.len() / 2, new_data);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run Add to Middle is {:?} (ns)", diff.num_nanoseconds().unwrap());
    data

}

fn add_to_end(mut data: Vec<Data>, new_data: Data) -> Vec<Data> {
    let start_time = Utc::now().time();
    data.insert(data.len() - 1, new_data);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run Add to End is {:?} (ns)", diff.num_nanoseconds().unwrap());
    data
}

fn remove_end(mut data: Vec<Data>) -> Vec<Data> {
    let start_time = Utc::now().time();
    data.remove(data.len() - 1);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run Remove End is {:?} (ns)", diff.num_nanoseconds().unwrap());
    data
}

fn remove_middle(mut data: Vec<Data>) -> Vec<Data> {
    let start_time = Utc::now().time();
    data.remove(data.len() / 2);
    let end_time = Utc::now().time();
    let diff = end_time - start_time;
    println!("Total time taken to run Remove Middle is {:?} (ns)", diff.num_nanoseconds().unwrap());
    data
}

#[derive(Debug, Clone)]
pub struct Data {
    tan: String,
    not: String,
    city: String,
    province: String,
    pi: String,
    ri: String,
}
