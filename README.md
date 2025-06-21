# config-load

The conditional configuration loader for Rust applications that use the `config` crate.

```toml
[dependencies]
config-load = "0.1"
serde = { version = "1", features = ["derive"] }
```

```rust
use std::path::Path;

use config_load::config::builder::DefaultState;
use config_load::config::{ConfigBuilder, Environment};
use config_load::{ConfigLoader, FileLocation, Load};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct AppConfig {
    enabled: bool,
    title: String,
    attempts: u32,
}

impl Load for AppConfig {
    fn load(config_builder: ConfigBuilder<DefaultState>) -> config_load::Result<Self> {
        let config = config_builder
            .add_source(Environment::with_prefix("APP").separator("_"))
            .build()?;
        config.try_deserialize()
    }
}

let config_file = None; // Parsed from command line arguments, for example

let config: AppConfig = ConfigLoader::default()
    .add(
        FileLocation::first_some_path()
            .from_env("APP_ROOT_CONFIG")
            .from_home(Path::new(".example_app").join("AppConfig.toml")),
    )
    .exclude_not_exists() // Exclude file pathes that don't exist, especially when APP_ROOT_CONFIG is set to ""
    .add(
        FileLocation::first_some_path()
            .from_file(config_file)
            .from_cwd_and_parents_exists("AppConfig.toml"),
    )
    .load()
    .expect("Failed to load config");
```
