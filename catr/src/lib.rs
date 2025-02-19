use std::error::Error;
use clap::Arg;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run(config: Config) -> MyResult<()> {
    dbg!(config);
    Ok(())
}

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = clap::App::new("catr")
        .version("0.1.0")
        .author("t k. <tk@gmail.com>")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("TEXT")
                .help("Input text")
                .required(true),
        )
        .arg(
            Arg::with_name("n_option")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("b_option")
                .short("b")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();


    let arg_files = matches.values_of_lossy("files").unwrap();
    let n_option = matches.is_present("n_option");
    let b_option = matches.is_present("b_option");


    let c = Config {
        files: arg_files,
        number_lines: n_option,
        number_nonblank_lines: b_option,
    };

    //if n_option == true && b_option == true { Err("--n-option and --b_option are mutually exclusive")} else { Ok(c) }
    if n_option && b_option {
        // `String`を`Box<dyn Error>`にキャスト
        Err(From::from("--number-nonblank-lines"))
    } else {
        Ok(c)
    }

}
