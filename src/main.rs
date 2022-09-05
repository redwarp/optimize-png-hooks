use clap::{App, Arg};
use oxipng::{optimize, InFile, Options, OutFile, PngResult};
use std::path::PathBuf;

fn optimize_png(path: &str, uses_zopfli: bool) -> PngResult<()> {
    let path_buf = PathBuf::from(path);

    let mut option = Options::from_preset(5);
    option.strip = oxipng::Headers::Safe;
    if uses_zopfli {
        option.deflate = oxipng::Deflaters::Zopfli;
    }

    optimize(&InFile::Path(path_buf), &OutFile::Path(None), &option)
}

fn main() {
    let matches = App::new("optimize png")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Optimize png using oxipng")
        .arg(Arg::new("zopfli").short('z').long("zopfli").help(
            "If present, will use zopfli for compression. \
            More efficient compression, but order of magnitude slower. Not recommended.",
        ))
        .arg(
            Arg::new("path")
                .help("Path(s) to files to optimize")
                .multiple_occurrences(true)
                .required(true),
        )
        .get_matches();

    let uses_zopfli = matches.is_present("zopfli");

    let results: Result<Vec<_>, _> = matches
        .values_of("path")
        .expect("Path was not set")
        .map(|path| optimize_png(path, uses_zopfli))
        .collect();

    std::process::exit(match results {
        Ok(_) => 0,
        Err(err) => {
            eprintln!("{}", err);
            1
        }
    });
}
