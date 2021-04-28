use std::path::PathBuf;

use clap::{App, Arg};
use oxipng::{optimize, InFile, Options, OutFile, PngResult};

fn optimize_png(path: &str) -> PngResult<()> {
    let path_buf = PathBuf::from(path);

    let mut option = Options::from_preset(3);
    option.strip = oxipng::Headers::Safe;

    optimize(&InFile::Path(path_buf), &OutFile::Path(None), &option)
}

fn main() {
    let matches = App::new("optimize png")
        .version(env!("CARGO_PKG_VERSION"))
        .about("optimize png using oxipng")
        .arg(
            Arg::with_name("path")
                .help("path(s) to files to optimize")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let results: Result<Vec<_>, _> = matches
        .values_of("path")
        .unwrap()
        .map(|path| optimize_png(path))
        .collect();

    std::process::exit(match results {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
