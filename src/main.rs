use clap::{App, Arg};
use reqwest;
use std::str;

fn build_gitignore_url(pl: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
        pl
    )
}

fn build_makefile_url(pl: &str) -> String {
    format!(
        "https://raw.githubusercontent.com/bitemyapp/makefiles/master/{}.makefile",
        pl
    )
}

async fn get_from_github(url: &str) -> Result<(), reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    println!("{}", body);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    let matches = App::new("lefortovo")
        .version("1.1")
        .author("Chris Allen <cma@bitemyapp.com>")
        .about("For putting things in a hole where they are forgotten")
        .arg(
            Arg::with_name("gitignore")
                .short("g")
                .long("gitignore")
                .value_name("gitignore")
                .help("Fetch a gitignore file from github/gitignore")
                .conflicts_with("makefile")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("makefile")
                .short("m")
                .long("makefile")
                .value_name("makefile")
                .help("Fetch a makefile from bitemyapp/makefiles")
                .conflicts_with("gitignore")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("lang")
                .short("l")
                .long("lang")
                .value_name("proglang")
                .help("The language whose gitignore you want")
                .required(true)
                .takes_value(true),
        )
        .get_matches();
    let lang = matches.value_of("lang").unwrap();
    let gitignore = matches.is_present("gitignore");
    let makefile = matches.is_present("makefile");

    let url: String = match (gitignore, makefile) {
        (true, true) => panic!("You must pick gitignore or makefile, not both!"),
        (true, false) => build_gitignore_url(lang).parse().unwrap(),
        (false, true) => build_makefile_url(lang).parse().unwrap(),
        (false, false) => build_gitignore_url(lang).parse().unwrap(),
    };

    get_from_github(&url).await?;
    Ok(())
}
