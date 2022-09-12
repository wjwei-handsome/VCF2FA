use std::{
    env,
    io::{stdout, Write},
    process::{self, Command},
    string,
};

// fn main() {
//     // Get the arguments
//     let args: Vec<String> = env::args().collect();
//     // println!("{:?}", args);
//     let query = &args[1];
//     let filename = &args[2];
//     println!("Searching for {}", query);
//     println!("In file {}", filename);

//     let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

//     println!("With text:\n{}", contents);
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });
    println!("fa filename is {}", config.fa_filename);
    println!("vcf filename is {}", config.vcf_filename);

    run(config);
}

// fn parse_config(args: &[String]) -> Config {
//     let query = &args[1].clone();
//     let filename = &args[2].clone();
//     Config(query, filename)
// }

struct Config {
    fa_filename: String,
    vcf_filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let fa_filename = args[1].clone();
        let vcf_filename = args[2].clone();
        Ok(Config {
            fa_filename,
            vcf_filename,
        })
    }
}

fn run(config: Config) {
    // let contents =
    //     fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    // println!("With text:\n{}", contents);
    let query_list = get_querylist_from_file(&config.vcf_filename);
    let mut final_output_list = Vec::new();
    for query in query_list {
        let output = get_consensus(&config.fa_filename, &config.vcf_filename, &query);
        final_output_list.push(output);
    }
    stdout()
        .write_all(final_output_list.join("\n").as_bytes())
        .unwrap();
}

fn get_querylist_from_file(vcf_filename: &String) -> Vec<String> {
    let query_output = Command::new("bcftools")
        .arg("query")
        .arg("-l")
        .arg(vcf_filename)
        .output()
        .expect("failed to execute process");
    let query_list = string::String::from_utf8_lossy(&query_output.stdout);
    let query_list: Vec<String> = query_list
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    query_list
}

fn get_consensus(fa_filename: &String, vcf_filename: &String, query: &String) -> String {
    let grepout = Command::new("bcftools")
        .arg("consensus")
        .arg("-f")
        .arg(fa_filename)
        .arg(vcf_filename)
        .arg("-s")
        .arg(query)
        .arg("-H1")
        .output()
        .expect("failed to execute process");
    let result = String::from_utf8(grepout.stdout);
    result.unwrap()
}
