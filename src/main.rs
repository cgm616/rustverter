extern crate clap;
use clap::{Arg, App};

fn main() {
    let matches = App::new("Rustverter")
                           .version("0.0.1")
                           .author("cgm616 <cgm616@me.com>")
                           .about("Simple rust CLI to convert between computer number encodings")
                           .arg(Arg::with_name("to")
                                .short("t")
                                .long("to")
                                .value_name("ENCODING")
                                .help("Sets encoding to convert to, either `binary`, `hex`, `decimal`, or `octa`")
                                .required(true)
                                .takes_value(true))
                           .arg(Arg::with_name("from")
                                .short("f")
                                .long("from")
                                .value_name("ENCODING")
                                .help("Sets encoding to convert from, either `binary`, `hex`, `decimal`, or `octa`")
                                .required(true)
                                .takes_value(true))
                           .arg(Arg::with_name("VALUE")
                                .help("Sets value to convert")
                                .required(true)
                                .index(1))
                           .get_matches();

    let value = matches.value_of("VALUE").unwrap();
    let to = matches.value_of("to").unwrap();
    let from = matches.value_of("from").unwrap();

    let output = normalize(value, from);
    
    print_encoding(output, to);
}

fn normalize(input: &str, encoding: &str) -> i64 {
    let mut output = Ok(0);
    
    match encoding {
        "binary" => {
            output = i64::from_str_radix(input, 2);
        },
        "hex" => {
            output = i64::from_str_radix(input, 16);
        },
        "octa" => {
            output = i64::from_str_radix(input, 8);
        },
        "decimal" => {
            output = input.parse::<i64>();
        },
        _ => error_arg(),
    }

    match output {
        Ok(i) => { i },
        Err(err) => panic!("Conversion failed, check input. Error: {:?}", err),
    }
}

fn print_encoding(output: i64, encoding: &str) {
    match encoding {
        "binary" => println!("{:b}", output),
        "hex" => println!("{:X}", output),
        "octa" => println!("{:o}", output),
        "decimal" => println!("{}", output),
        _ => error_arg(),
    }
}

fn error_arg() {
    panic!("Encoding not recognized. Run with `--help` or `-h` for more information.");
}
