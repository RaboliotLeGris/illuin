extern crate clap;

#[derive(Debug)]
pub struct AppConfig {
    pub port: u16,
    pub storage_path: String,
    pub tls: bool,
    pub base_route: String,
}

impl AppConfig {
    fn new(cli_args: clap::ArgMatches) -> Self {
        AppConfig {
            port: u16::from_str_radix(cli_args.value_of("port").unwrap(), 10).unwrap_or(8080),
            storage_path: cli_args.value_of("storage_path").unwrap().to_string(),
            tls: cli_args.is_present("tls"),
            base_route: cli_args.value_of("base_route").unwrap().to_string(),
        }
    }
}

pub fn get_config() -> AppConfig {
    let default_folder = get_tmp_folder();
    let cli_args = clap::App::new("Illuin")
        .version("0.1.0")
        .author("Jordan P. <jordan@raboland.fr>")
        .about("Serving images!")
        .arg(clap::Arg::with_name("port")
            .help("Use a the listen port of the app")
            .short("p")
            .long("port")
            .value_name("port")
            .default_value("8080")
            .takes_value(true))
        .arg(clap::Arg::with_name("storage_path")
            .help("Path where the image are stored")
            .short("s")
            .long("storage-path")
            .value_name("storage_path")
            .default_value(&default_folder)
            .takes_value(true))
        .arg(clap::Arg::with_name("tls")
            .help("return path that are compatible with TLS")
            .takes_value(false)
            .long("tls"))
        .arg(clap::Arg::with_name("base_route")
            .help("Prepend all routes with provided value. Format must be like: '/base/'")
            .long("base-route")
            .value_name("base_route")
            .default_value("/")
            .takes_value(true))
        .get_matches();

    AppConfig::new(cli_args)
}

fn get_tmp_folder() -> String {
    let mut default_dir = std::env::temp_dir();
    default_dir.push("illuin");
    default_dir.into_os_string().into_string().unwrap()
}
