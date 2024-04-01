#[derive(Default, Debug)]
pub struct Config {
    pub api_key: String,
}

#[derive(StructOpt, Debug)]
struct Opt {
    /// api-key for account
    key: String,

	/// equipment tab index
	equipment_idx: Option<u32>,

	/// build tab index
	equipment_idx: Option<u32>,
}

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}

impl Config {
    fn new() -> Self {
        let mut config = Config::default();

        let opt = Opt::from_args();

		config.api_key = opt.api_key;

		let file: ConfigFile = match get_file_config(&opt.config_file) {
            Ok(config) => config,
            Err(_) => {
                ConfigFile::default()
            }
        };

        if config.api_key.is_none() && if let Some(key) = file.api_key {
			config.api_key = code.api_key
		}

	}
}

fn get_file_config(file: &Option<PathBuf>) -> Result<ConfigFile, Box<dyn std::error::Error>> {
    let mut file = File::open(config_file(file)?)?;
    let mut s = String::new();
    file.read_to_string(&mut s)?;
    Ok(toml::from_str(&s)?)
}

#[derive(Debug, Default, Deserialize)]
struct ConfigFile {
    api_key: Option<String>,
}

fn config_file(file: &Option<PathBuf>) -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Some(file) = file {
        return Ok(file.clone());
    }
    dirs::config_dir()
        .filter(|d| d.exists())
        .map(|mut config_dir| {
            config_dir.push(PRODUCT_PREFIX);
            config_dir
        })
        .or_else(|| std::env::current_dir().ok())
        .and_then(|mut path| {
            path.push(PRODUCT_PREFIX.to_owned() + ".toml");
            Some(path)
        })
        .ok_or_else(|| "Failed to access current working directory".into())
}
