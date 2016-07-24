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

    let mut output = Ok(0);
    
    match from {
        // Converts all types to store in a i64
        "binary" => {
            output = i64::from_str_radix(value, 2);
        },
        "hex" => {
            output = i64::from_str_radix(value, 16);
        },
        "octa" => {
            output = i64::from_str_radix(value, 8);
        },
        "decimal" => {
            output = value.parse::<i64>();
        },
        _ => error_arg(),
    }

    let output = match output {
        Ok(i) => { i },
        Err(err) => panic!("Conversion failed, check input. Error: {:?}", err),
    };

    match to {
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
