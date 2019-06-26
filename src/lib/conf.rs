use config;

pub fn get_conf(mrkvconf: String) -> config::Config {
    let mut settings = config::Config::default();
    let settings = settings
        // Add in `./Settings.toml`
        .merge(config::File::with_name(&mrkvconf))
        .unwrap()
        // Add in settings from the environment (with a prefix of MRKV)
        // Eg.. `MRKV_DEBUG=1 ./target/MRKV` would set the `debug` key
        .merge(config::Environment::with_prefix("MRKV"))
        .unwrap();
    settings.clone()
}
