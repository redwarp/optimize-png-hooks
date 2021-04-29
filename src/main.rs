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
        .arg(Arg::with_name("zopfli").short("z").long("zopfli").help(
            "If present, will use zopfli for compression. \
            More efficient compression, but order of magnitude slower. Not recommanded.",
        ))
        .arg(
            Arg::with_name("path")
                .help("Path(s) to files to optimize")
                .multiple(true)
                .required(true),
        )
        .get_matches();

    let uses_zopfli = matches.is_present("zopfli");
    let results: Result<Vec<_>, _> = matches
        .values_of("path")
        .unwrap()
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
