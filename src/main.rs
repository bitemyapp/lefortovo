extern crate clap;
extern crate hyper;

use clap::{Arg, App};
use hyper::Client;
use std::io::prelude::*;

fn build_url(pl: &str) -> String {
    format!("https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
            pl)
}

fn get_from_github(lang: &str) {
    let client = Client::new();
    let url = build_url(lang);
    let mut res = client.get(&url[..])
        .send()
        .unwrap();
    let mut s = String::new();
    let _ = res.read_to_string(&mut s);
    println!("{}", s);
}

fn main() {
    let matches = App::new("lefortovo")
        .version("1.0")
        .author("Chris Allen <cma@bitemyapp.com>")
        .about("For putting things in a hole where they are forgotten")
        .arg(Arg::with_name("lang")
            .short("l")
            .long("lang")
            .value_name("proglang")
            .help("The language whose gitignore you want")
            .required(true)
            .takes_value(true))
        .get_matches();
    let lang = matches.value_of("lang").unwrap();
    get_from_github(lang);
}
