use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

use either::Either;

use crate::location::Location;

pub enum FileLocation {
    FirstSome(Option<PathBuf>),
}

impl FileLocation {
    pub fn first_some_path() -> Self {
        Self::FirstSome(None)
    }

    pub fn from_file(mut self, file_path: Option<PathBuf>) -> Self {
        if let Some(file_path) = file_path {
            if file_path.is_relative() {
                return self.from_cwd(file_path);
            } else {
                match &mut self {
                    FileLocation::FirstSome(path) => {
                        if path.is_none() {
                            path.replace(file_path);
                        }
                    },
                }
            }
        }
        self
    }

    pub fn from_file_exists(mut self, file_path: Option<PathBuf>) -> Self {
        if let Some(file_path) = file_path {
            if file_path.is_relative() {
                return self.from_cwd_exists(file_path);
            } else {
                match &mut self {
                    FileLocation::FirstSome(path) => {
                        if path.is_none() {
                            *path = existing_file(Some(file_path), Option::<&Path>::None);
                        }
                    },
                }
            }
        }
        self
    }

    pub fn from_env(mut self, env_var: impl AsRef<OsStr>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = env::var(env_var).ok().map(Into::into);
                }
            },
        }
        self
    }

    pub fn from_env_exists(mut self, env_var: impl AsRef<OsStr>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = existing_file(env::var(env_var).ok().map(Into::into), Option::<&Path>::None);
                }
            },
        }
        self
    }

    pub fn from_home(mut self, relative_path: impl AsRef<Path>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = env::home_dir().map(|home| home.join(relative_path));
                }
            },
        }
        self
    }

    pub fn from_home_exists(mut self, relative_path: impl AsRef<Path>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = existing_file(env::home_dir(), Some(relative_path));
                }
            },
        }
        self
    }

    pub fn from_cwd(mut self, relative_path: impl AsRef<Path>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = env::current_dir().ok().map(|cwd| cwd.join(relative_path));
                }
            },
        }
        self
    }

    pub fn from_cwd_exists(mut self, relative_path: impl AsRef<Path>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = existing_file(env::current_dir().ok(), Some(relative_path));
                }
            },
        }
        self
    }

    pub fn from_cwd_and_parents_exists(mut self, relative_path: impl AsRef<Path>) -> Self {
        match &mut self {
            FileLocation::FirstSome(path) => {
                if path.is_none() {
                    *path = env::current_dir()
                        .ok()
                        .and_then(|cwd| find_existing_file_in_dir_and_parents(cwd, relative_path));
                }
            },
        }
        self
    }
}

impl Location for FileLocation {
    fn try_into_path(self) -> Either<PathBuf, Self>
    where
        Self: Sized,
    {
        match self {
            Self::FirstSome(path) => path
                .map(Either::Left)
                .unwrap_or_else(|| Either::Right(Self::first_some_path())),
        }
    }
}

fn existing_file(root_path: Option<PathBuf>, relative_path: Option<impl AsRef<Path>>) -> Option<PathBuf> {
    root_path.and_then(|root| {
        let path = if let Some(relative_path) = relative_path {
            root.join(relative_path)
        } else {
            root
        };
        path.is_file().then_some(path)
    })
}

fn find_existing_file_in_dir_and_parents(dir: impl AsRef<Path>, relative_path: impl AsRef<Path>) -> Option<PathBuf> {
    let mut current_dir = dir.as_ref();
    loop {
        let path = current_dir.join(relative_path.as_ref());
        if path.is_file() {
            break Some(path);
        } else {
            current_dir = current_dir.parent()?;
        }
    }
}
