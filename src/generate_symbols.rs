use regex::bytes::RegexSet;
use regex::Regex;
use std::fs::File;
use std::io::copy;
use std::io::{self, Read};
use std::process;
use log::{info, trace, warn};


const URL_SRC: &str = "http://milde.users.sourceforge.net/LUCR/Math/data/unimathsymbols.txt";

pub async fn run() -> io::Result<()> {
    let response = match reqwest::get(URL_SRC).await {
        Ok(r) => r,
        Err(error) => panic!("GET Request Error: {:?}", error),
    };

    let mut dl = {
        let fname = response
            .url()
            .path_segments()
            .and_then(|segments| segments.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("latex_src.bin");
        info!("Latex symbol source file: {}", fname);
        let fname = format!("{}/{}", std::env!("CARGO_MANIFEST_DIR").to_string(), fname);
        info!("Downloaded to: '{:?}'", fname);
        File::create(fname)?
    };
    let content = match response.text().await {
        Ok(c) => {
            info!("Content read: {}", c);
            c
        },
        Err(error) => {
            warn!("Problem reading contents: {:?}", error);
            panic!()
        }
    };
    copy(&mut content.as_bytes(), &mut dl)?;
    Ok(())
}

#[tokio::main]
pub async fn main() {
    trace!("Starting generation of latex symbols");
    if let Err(err) = run().await {
        eprintln!("{}", err);
        process::exit(1);
    }
}

pub fn generate_symbols() {
    let re1 = Regex::new(r"^[\d]\w+\^(.)").unwrap();
    let re2 = Regex::new(r"(\\\w+)\{?(\\?\w+)(\}?)").unwrap();
    let mut latex_symbol_file = File::open(format!(
        "{}/{}",
        std::env!("CARGO_MANIFEST_DIR"),
        "unimathsymbols.txt"
    ))
    .expect("File may not exist!");
    let mut contents = String::new();
    match latex_symbol_file.read_to_string(&mut contents) {
        Ok(c) => c,
        Err(error) => panic!("Problem reading contents: {:?}", error),
    };
    for line in contents.lines() {
        for cap1 in re1.captures_iter(&line) {
            for cap2 in re2.captures_iter(&line){
                println!("{} = \\\\\\{}", cap1.get(0).unwrap().as_str().chars().last().unwrap(), cap2.get(0).unwrap().as_str());
            }
        }
    }
}
