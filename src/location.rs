use std::path::PathBuf;

use either::Either;

pub mod file;

pub trait Location {
    fn try_into_path(self) -> Either<PathBuf, Self>
    where
        Self: Sized;
}

impl<T: TryInto<PathBuf, Error = T>> Location for T {
    fn try_into_path(self) -> Either<PathBuf, Self> {
        self.try_into().map(Either::Left).unwrap_or_else(Either::Right)
    }
}
