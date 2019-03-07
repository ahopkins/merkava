use config;

pub fn get_conf(mrkvconf: String) -> config::Config {
    let mut settings = config::Config::default();
    let settings = settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name(&mrkvconf))
        .unwrap()
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .merge(config::Environment::with_prefix("APP"))
        .unwrap();
    settings.clone()
}
