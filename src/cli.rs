extern crate clap;

use std::path::Path;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub storage_path: String,
}

impl Config {
    fn new(cli_args: clap::ArgMatches) -> Self {
        Config {
            port: u16::from_str_radix(cli_args.value_of("port").unwrap(), 10).unwrap_or(8080),
            storage_path: cli_args.value_of("storage_path").unwrap().to_string(),
        }
    }
}

pub fn get_config() -> Config {
    let cli_args = clap::App::new("Illuin")
        .version("0.1.0")
        .author("Jordan P. <jordan@raboland.fr>")
        .about("Serving images!")
        .arg(clap::Arg::with_name("port")
            .short("p")
            .long("port")
            .value_name("port")
            .default_value("8080")
            .help("Use a the listen port of the app")
            .takes_value(true))
        .arg(clap::Arg::with_name("storage_path")
            .short("s")
            .long("storage-path")
            .value_name("storage_path")
            .default_value(Path::new("/tmp/illuin").as_os_str().to_str().unwrap())
            .help("Path where the image are stored")
            .takes_value(true))
    .get_matches();

    Config::new(cli_args)
}