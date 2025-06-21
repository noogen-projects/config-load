use config::ConfigBuilder;
use config::builder::{BuilderState, DefaultState};

pub trait Load<S: BuilderState = DefaultState> {
    fn load(config_builder: ConfigBuilder<S>) -> crate::Result<Self>
    where
        Self: Sized;
}
