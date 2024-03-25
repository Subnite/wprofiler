use std::env;
use std::fs;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::str::FromStr;
use std::fmt::Debug;
use num::traits::{Num, NumOps};

fn main() {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();

    // Check if an argument is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <\"filename.extention\">", args[0]);
        std::process::exit(1);
    }

    // Get the filename from the arguments
    let filename = &args[1];
    
    println!("[INFO] Reading from \"{}\" ...", filename);

    // Read the file
    match fs::read_to_string(filename) {
        Ok(contents) => {
            // Do something with the file contents
            match process_file(&contents) {
                Err(e) => {
                    eprintln!("Couldn't process the file! Err: {}", e);
                    std::process::exit(1);
                },
                _ => {
                    println!("[SUCCESS] Outputted to \"formatted.json\" in the same directory!");
                },
            }
        }
        Err(err) => {
            eprintln!("Error reading file {}: {}", filename, err);
            std::process::exit(1);
        }
    }
}

fn process_file(contents: &String) -> Result<(), &str> {
    let mut tabs = seperate_tabs(contents);
    let mut data_start_idx: usize = 0;

    // Print each part
    for (i, part) in tabs.iter().enumerate() {
        if part.contains("\n") {
            // println!("Data starts at part: {}", i);
            data_start_idx = i;
            tabs[i] = part.trim();
            break;
        }
    }

    if data_start_idx == 0 { return Err("Couldn't find where the data starts!"); }
    
    let (words, data) = tabs.split_at(data_start_idx);
    let entries = create_entries(&words, &data).unwrap();

    let _outputted = output_entries("formatted.json".to_string(), &entries);

    Ok(())
}

fn seperate_tabs(contents: &String) -> Vec<&str> {
    let parts: Vec<&str> = contents.split('\t').collect();
    parts
}

struct EntryData {
    _data: Vec<f64>,
    min: f64,
    max: f64,
    avg: f64,
}

struct Entry {
    name: String,
    data: Option<EntryData>,
}

impl Entry {
    fn new(name: String, data: EntryData) -> Self {
        let entry = Entry{name, data: Some(data)};
        entry
    }
}

impl EntryData
{
    fn new(data: Vec<f64>)->Self{
        let mut min: f64 = data[0];
        let mut max: f64 = data[0];
        let mut avg: f64 = 0.0;

        for &point in &data {
            if point < min {
                min = point;
            } else if point > max {
                max = point;
            }
            avg += point;
        }

        let size = data.len() as f64;
        avg = avg / size;
        EntryData{_data: data, min, max, avg}
    }
}

fn create_entries(words: &[&str], data: &[&str]) -> Result<Vec<Entry>, String> {
    let mut entries = Vec::new();
    let data_per_type = data.len() / words.len();
    println!("[INFO] Data Points: {} | Columns: {} | Rows: {}", data.len(), words.len(), data_per_type);

    for (i, word) in words.iter().enumerate() {
        // check if the data contains a number
        let num = data[i].parse::<f64>();
        match num {
            Ok(_) => {
                let collected_data = collect_data::<f64>(&data, i, words.len());
                let entrydata = EntryData::new(collected_data);
                entries.push(Entry::new(word.to_string(), entrydata));
            },
            _ => (),
        }
    }

    if entries.len() == 0 {
        return Err(String::from("No numeric data was found"));
    }

    Ok(entries)
}

fn collect_data<T>(data: &[&str], start_offset: usize, columns_amt: usize) -> Vec<T> 
where 
    T: Num + NumOps,
    T: FromStr, <T as FromStr>::Err: Debug
{
    let mut collected_data: Vec<T> = Vec::new();
    collected_data.reserve(data.len() / columns_amt);

    let mut i = start_offset;

    let mut num_str: &str;
    let mut num: T;
    while i < data.len() {
        num_str = data[i].trim();
        let parsed = num_str.parse::<T>();
        match parsed {
            Ok(r) => {
                num = r;
                collected_data.push(num);
            },
            Err(_) => {
                //eprintln!("str: {}, index: {}", num_str, i);
            }
        }

        i += columns_amt;
    }

    collected_data
}

fn output_entries(filename: String, entries: &Vec<Entry>) -> Result<(), ()>{
    let file = File::create(filename).unwrap();

    let mut writer = BufWriter::new(file);

    write_line(&mut writer, "{");

    for (i, entry) in entries.iter().enumerate() {
        let name = "\t\"".to_string() + entry.name.as_str()+ "\": {";
        write_line(&mut writer, name.as_str());

        let d = &entry.data;
        match d {
            Some(data) => {
                let min = "\t\t".to_string() + "\"minimum\": " + &data.min.to_string() + ",";
                let max = "\t\t".to_string() + "\"maximum\": " + &data.max.to_string() + ",";
                let avg = "\t\t".to_string() + "\"average\": " + &data.avg.to_string();

                write_line(&mut writer, min.as_str());
                write_line(&mut writer, max.as_str());
                write_line(&mut writer, avg.as_str());
            },
            None => (),
        }
        let mut end = ",";
        if i == entries.len()-1{
            end = ""
        }    

        write_line(&mut writer, &("\t}".to_string() + end));
    }

    write_line(&mut writer, "}");

    Ok(())
}

fn write_line(writer: &mut BufWriter<File>, line: &str) {
    writer.write_all(line.as_bytes()).unwrap();
    writer.write_all(b"\n").unwrap();
}
