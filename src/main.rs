extern crate clap;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate tokio_core;

use clap::{Arg, App};
use futures::{Future, Stream};

use std::str;

fn build_url(pl: &str) -> String {
    format!("https://raw.githubusercontent.com/github/gitignore/master/{}.gitignore",
            pl)
}

fn get_from_github(lang: &str) {
    let mut core = tokio_core::reactor::Core::new().unwrap();
    let handle = core.handle();
    let client = hyper::Client::configure()
        .connector(hyper_tls::HttpsConnector::new(4, &handle).unwrap())
        .build(&handle);

    let url = build_url(lang).parse().unwrap();
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
