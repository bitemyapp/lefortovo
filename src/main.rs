extern crate clap;
extern crate hyper;

use clap::{Arg, App};
use hyper::Client;
use std::io::prelude::*;

static PROG_LANGS: [&'static str; 122] = ["Actionscript",
                                          "Ada",
                                          "Agda",
                                          "Android",
                                          "AppceleratorTitanium",
                                          "AppEngine",
                                          "ArchLinuxPackages",
                                          "Autotools",
                                          "CakePHP",
                                          "CFWheels",
                                          "C",
                                          "Cpp", // C++
                                          "ChefCookbook",
                                          "Clojure",
                                          "CMake",
                                          "CodeIgniter",
                                          "CommonLisp",
                                          "Composer",
                                          "Concrete5",
                                          "Coq",
                                          "CraftCMS",
                                          "CUDA",
                                          "Dart",
                                          "Delphi",
                                          "D",
                                          "DM",
                                          "Drupal",
                                          "Eagle",
                                          "Elisp",
                                          "Elixir",
                                          "Elm",
                                          "EPiServer",
                                          "Erlang",
                                          "ExpressionEngine",
                                          "ExtJs",
                                          "Fancy",
                                          "Finale",
                                          "ForceDotCom",
                                          "Fortran",
                                          "FuelPHP",
                                          "Gcov",
                                          "GitBook",
                                          "Go",
                                          "Gradle",
                                          "Grails",
                                          "GWT",
                                          "Haskell",
                                          "Idris",
                                          "IGORPro",
                                          "Java",
                                          "Jboss",
                                          "Jekyll",
                                          "Joomla",
                                          "Julia",
                                          "KiCad",
                                          "Kohana",
                                          "LabVIEW",
                                          "Laravel",
                                          "Leiningen",
                                          "LemonStand",
                                          "Lilypond",
                                          "Lithium",
                                          "Lua",
                                          "Magento",
                                          "Maven",
                                          "Mercury",
                                          "MetaProgrammingSystem",
                                          "Nanoc",
                                          "Nim",
                                          "Node",
                                          "ObjectiveC", // Objective-C
                                          "OCaml",
                                          "Opa",
                                          "OpenCart",
                                          "OracleForms",
                                          "Packer",
                                          "Perl",
                                          "Phalcon",
                                          "PlayFramework",
                                          "Plone",
                                          "Prestashop",
                                          "Processing",
                                          "Python",
                                          "Qooxdoo",
                                          "Qt",
                                          "Rails",
                                          "R",
                                          "RhodesRhomobile",
                                          "ROS",
                                          "Ruby",
                                          "Rust",
                                          "Sass",
                                          "Scala",
                                          "Scheme",
                                          "SCons",
                                          "Scrivener",
                                          "Sdcc",
                                          "SeamGen",
                                          "SketchUp",
                                          "Smalltalk",
                                          "Stella",
                                          "SugarCRM",
                                          "Swift",
                                          "Symfony",
                                          "SymphonyCMS",
                                          "Terraform",
                                          "TeX",
                                          "Textpattern",
                                          "TurboGears2",
                                          "Typo3",
                                          "Umbraco",
                                          "Unity",
                                          "UnrealEngine",
                                          "VisualStudio",
                                          "VVVV",
                                          "Waf",
                                          "WordPress",
                                          "Xojo",
                                          "Yeoman",
                                          "Yii",
                                          "ZendFramework",
                                          "Zephir"];

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
