/*!
The conditional configuration loader for Rust applications that use the `config` crate.

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
 */
use std::marker::PhantomData;
use std::path::PathBuf;

pub use config;
use config::builder::{AsyncState, BuilderState, DefaultState};
use config::{Config, ConfigBuilder, ConfigError};
use either::Either;

pub use crate::load::Load;
pub use crate::location::Location;
pub use crate::location::file::FileLocation;

pub mod load;
pub mod location;

pub type Result<T> = std::result::Result<T, ConfigError>;

#[derive(Debug, Clone)]
pub struct ConfigLoader<S: BuilderState = DefaultState> {
    config_paths: Vec<PathBuf>,
    _state: PhantomData<S>,
}

impl<S: BuilderState> ConfigLoader<S> {
    pub fn new() -> Self {
        Self {
            config_paths: Vec::new(),
            _state: PhantomData,
        }
    }

    pub fn add(mut self, location: impl Location) -> Self {
        match location.try_into_path() {
            Either::Left(path) => self.config_paths.push(path),
            Either::Right(_) => (),
        }
        self
    }

    pub fn exclude_not_exists(mut self) -> Self {
        self.config_paths.retain(|path| path.is_file());
        self
    }
}

impl Default for ConfigLoader<DefaultState> {
    fn default() -> Self {
        Self::new_default()
    }
}

impl ConfigLoader<DefaultState> {
    pub fn new_default() -> Self {
        Self::new()
    }

    pub fn builder(self) -> ConfigBuilder<DefaultState> {
        let mut config_builder = Config::builder();
        for path in self.config_paths {
            config_builder = config_builder.add_source(config::File::from(path))
        }
        config_builder
    }

    pub fn load<T: Load<DefaultState>>(self) -> Result<T> {
        let config_builder = self.builder();
        T::load(config_builder)
    }
}

impl ConfigLoader<AsyncState> {
    pub fn new_async() -> Self {
        Self::new()
    }

    pub fn builder(self) -> ConfigBuilder<AsyncState> {
        let mut config_builder = ConfigBuilder::<AsyncState>::default();
        for path in self.config_paths {
            config_builder = config_builder.add_source(config::File::from(path))
        }
        config_builder
    }

    pub fn load<T: Load<AsyncState>>(self) -> Result<T> {
        let config_builder = self.builder();
        T::load(config_builder)
    }
}
