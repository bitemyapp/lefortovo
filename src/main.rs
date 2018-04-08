extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use clap::{Arg, App};
use futures::{Future, Stream};
// use futures::future::Future;

use std::str;

fn build_gitignore_url(pl: &str) -> String {
    format!("https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
            pl)
}

fn build_makefile_url(pl: &str) -> String {
    format!("https://raw.githubusercontent.com/bitemyapp/makefiles/master/{}.makefile",
            pl)
}

fn get_from_github(url: hyper::Uri) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);
    let mut s = String::new();
    {
        let work = client.get(url).and_then(|res| {
            res.body().for_each(|chunk| {
                s.push_str(str::from_utf8(&*chunk).unwrap());
                futures::future::ok(())
            })
        });
        core.run(work).unwrap();
    }
    println!("{}", s);
}

fn main() {
    let matches = App::new("lefortovo")
        .version("1.0")
        .author("Chris Allen <cma@bitemyapp.com>")
        .about("For putting things in a hole where they are forgotten")
        .arg(Arg::with_name("gitignore")
             .short("g")
             .long("gitignore")
             .value_name("gitignore")
             .help("Fetch a gitignore file from github/gitignore")
             .conflicts_with("makefile")
             .takes_value(false))
        .arg(Arg::with_name("makefile")
             .short("m")
             .long("makefile")
             .value_name("makefile")
             .help("Fetch a makefile from bitemyapp/makefiles")
             .conflicts_with("gitignore")
             .takes_value(false))
        .arg(Arg::with_name("lang")
            .short("l")
            .long("lang")
            .value_name("proglang")
            .help("The language whose gitignore you want")
            .required(true)
            .takes_value(true))
        .get_matches();
    let lang = matches.value_of("lang").unwrap();
    let gitignore = matches.is_present("gitignore");
    let makefile = matches.is_present("makefile");

    let url = match (gitignore, makefile) {
        (true, true) => panic!("You must pick gitignore or makefile, not both!"),
        (true, false) => build_gitignore_url(lang).parse().unwrap(),
        (false, true) => build_makefile_url(lang).parse().unwrap(),
        (false, false) => build_gitignore_url(lang).parse().unwrap(),
    };

    get_from_github(url);
}
