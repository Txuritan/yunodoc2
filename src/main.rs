extern crate clap;
extern crate ini;
extern crate liquid;
extern crate regex;


use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::io::prelude::*;


use clap::{Arg, App};
use ini::Ini;
use liquid::{Renderable, Context, Value};
use regex::Regex;


fn main() {


    let l_style_ian = "[Table]
separator = \"# ----{{variable}}---- # ----{{type}}---- # ----{{purpose}}---- #\"
empty = \"#     {{variable}}     #     {{type}}     #     {{purpose}}     #\"

[Title]
variable = \"~Variables~\"
type = \"~Type~\"
purpose = \"~Purpose~\"";


    // Args Parsing
    let l_arg_matches = App::new("yunodoc")
        .version("2.0.0")
        .author("Ian Cronkright <txurtian@yahoo.com>")
        .about("Docs some things")
        .arg(Arg::with_name("STYLE")
            .short("s")
            .long("style")
            .value_name("STYLE")
            .help("Table output style")
            .takes_value(true)
        )
        /*.arg(Arg::with_name("INLINE")
            .short("i")
            .long("inline")
            .value_name("INLINE")
            .help("Uses build in styles without making a file")
            .takes_value(false)
        )*/
        .arg(Arg::with_name("GENERATE")
            .short("g")
            .long("generate")
            .value_name("GENERATE")
            .help("Generates built in styles")
            .takes_value(false)
        )
        .arg(Arg::with_name("FILE")
            .help("Code source file")
            //.required(true)
            .index(1)
        )
        .get_matches();


    if l_arg_matches.is_present("GENERATE") {


        let mut l_style_file_ian = match File::create("styles/ian.ini") {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        match l_style_file_ian.write_all(&l_style_ian.as_bytes()) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };


    } else {


        let l_arg_file = l_arg_matches.value_of("FILE").unwrap();
        let l_arg_style = l_arg_matches.value_of("STYLE").unwrap();


        // Style Load
        let l_style_config = Ini::load_from_file("styles/".to_owned() + l_arg_style + ".ini").unwrap();

        let l_table_section = l_style_config.section(Some("Table".to_owned())).unwrap();

        let l_table_separator = l_table_section.get("separator").unwrap();
        let l_table_empty = l_table_section.get("empty").unwrap();

        let l_table_section = l_style_config.section(Some("Title".to_owned())).unwrap();

        let l_title_variable = l_table_section.get("variable").unwrap();
        let l_title_type = l_table_section.get("type").unwrap();
        let l_title_purpose = l_table_section.get("purpose").unwrap();


        // Code Load
        let l_file = match File::open(l_arg_file) {
            Ok(file) => file,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };

        let l_file_buffer = BufReader::new(&l_file);


        // Regex Compile
        let l_doc_reg = match Regex::new(r"((#|//) DOC .*)") {
            Ok(r) => r,
            Err(e) => {
                println!("Could not compile regex: {}", e);
                return;
            }
        };

        let l_var_reg = match Regex::new(r"VAR (.*?) TYP") {
            Ok(r) => r,
            Err(e) => {
                println!("Could not compile regex: {}", e);
                return;
            }
        };

        let l_typ_reg = match Regex::new(r"TYP (.*?) PUR") {
            Ok(r) => r,
            Err(e) => {
                println!("Could not compile regex: {}", e);
                return;
            }
        };

        let l_pur_reg = match Regex::new(r"PUR (.*)") {
            Ok(r) => r,
            Err(e) => {
                println!("Could not compile regex: {}", e);
                return;
            }
        };

        let mut l_var_vec: Vec<String> = Vec::new();
        let mut l_typ_vec: Vec<String> = Vec::new();
        let mut l_pur_vec: Vec<String> = Vec::new();


        // Match Lines
        for i_line in l_file_buffer.lines() {
            let l_line = i_line.unwrap();

            if l_doc_reg.is_match(&l_line) {
                for i_var_cap in l_var_reg.captures_iter(&l_line) {
                    &l_var_vec.push(i_var_cap[1].to_string());
                }

                for i_typ_cap in l_typ_reg.captures_iter(&l_line) {
                    &l_typ_vec.push(i_typ_cap[1].to_string());
                }

                for i_pur_cap in l_pur_reg.captures_iter(&l_line) {
                    &l_pur_vec.push(i_pur_cap[1].to_string());
                }
            }
        }


        // Check Length
        let mut l_var_longest = longest(&l_var_vec);

        if l_var_longest < l_title_variable.len() {
            l_var_longest = l_title_variable.len();
        }

        let mut l_typ_longest = longest(&l_typ_vec);

        if l_typ_longest < l_title_type.len() {
            l_typ_longest = l_title_type.len();
        }

        let mut l_pur_longest = longest(&l_pur_vec);

        if l_pur_longest < l_title_purpose.len() {
            l_pur_longest = l_title_purpose.len();
        }


        // Compile Table
        let mut l_table = String::from("");


        // Seperator
        let mut l_seperator_context = Context::new();
        l_seperator_context.set_val("variable", Value::str(format!("{:-<1$}", "", l_var_longest).as_str()));
        l_seperator_context.set_val("type", Value::str(format!("{:-<1$}", "", l_typ_longest).as_str()));
        l_seperator_context.set_val("purpose", Value::str(format!("{:-<1$}", "", l_pur_longest).as_str()));

        let l_seperator_template = liquid::parse(l_table_separator, Default::default()).unwrap();

        let l_seperator_output = l_seperator_template.render(&mut l_seperator_context);

        let l_seperator_string : String = l_seperator_output.unwrap().unwrap();
        l_table = format!("{}{}\n", l_table, &l_seperator_string);


        // Header
        let mut l_header_context = Context::new();
        l_header_context.set_val("variable", Value::str(format!("{word:<spaces$}", word=l_title_variable, spaces=l_var_longest).as_str()));
        l_header_context.set_val("type", Value::str(format!("{word:<spaces$}", word=l_title_type, spaces=l_typ_longest).as_str()));
        l_header_context.set_val("purpose", Value::str(format!("{word:<spaces$}", word=l_title_purpose, spaces=l_pur_longest).as_str()));

        let l_header_template = liquid::parse(l_table_empty, Default::default()).unwrap();

        let l_header_output = l_header_template.render(&mut l_header_context);

        l_table = format!("{}{}\n", l_table, l_header_output.unwrap().unwrap());


        // Add Seperator
        l_table = format!("{}{}\n", l_table, &l_seperator_string);


        // Variables
        for (i, _k) in l_var_vec.iter().enumerate() {
            let mut l_variable_context = Context::new();
            l_variable_context.set_val("variable", Value::str(format!("{word:<spaces$}", word=&l_var_vec[i], spaces=l_var_longest).as_str()));
            l_variable_context.set_val("type", Value::str(format!("{word:<spaces$}", word=&l_typ_vec[i], spaces=l_typ_longest).as_str()));
            l_variable_context.set_val("purpose", Value::str(format!("{word:<spaces$}", word=&l_pur_vec[i], spaces=l_pur_longest).as_str()));

            let l_variable_template = liquid::parse(l_table_empty, Default::default()).unwrap();

            let l_variable_output = l_variable_template.render(&mut l_variable_context);

            l_table = format!("{}{}\n", l_table, l_variable_output.unwrap().unwrap());
        }


        // Add Seperator
        l_table = format!("{}{}", l_table, &l_seperator_string);


        println!("{}", l_table);
    }


}



fn longest(i_vector: &Vec<String>) -> usize {
    let mut max: usize = 0;

    for i_line in i_vector.into_iter() {
        if &i_line.len() > &max {
            max = i_line.len();
        }
    }

    return max;
}