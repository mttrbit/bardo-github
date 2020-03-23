use std::env;
use std::path::PathBuf;
use std::path::Path;
use std::fs::File;
use std::io;
use std::io::{Read,Write};
use toml::Value;

pub use io::Result;

/// Read a file into `Vec<u8>` from given path.
/// The path can be a `String` or a `Path`.
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    if let Ok(meta) = file.metadata() {
        data.reserve(meta.len() as usize);
    }
    file.read_to_end(&mut data)?;
    Ok(data)
}

/// Read an UTF-8 encoded file into `String` from given path.
/// The path can be a `String` or `Path`.
pub fn read_str<P: AsRef<Path>>(path: P) -> Result<String> {
    let bytes = read(path)?;
    String::from_utf8(bytes).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "file encoding is not UTF-8")
    })
}

/// Read toml file into `Value` from given path.
/// The path can be `String` or `Path`.
pub fn read_toml<P: AsRef<Path>>(path: P) -> Result<Value> {
    let bytes = read(path)?;
    toml::from_slice(&bytes).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidData, "file content is no valid toml")
    })
}

/// Creates a file at the given path with contents of `Vec<u8>` or `&[u8]`, etc.
/// Overwrites, non-atomically, if the file exists.
/// The path can be `String` or `Path`
pub fn write<P: AsRef<Path>, Bytes: AsRef<[u8]>>(path: P, data: Bytes) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(data.as_ref())?;
    Ok(())
}

/// Creates a file at the given path with given text contents, encoded as UTF-8.
/// Overwrites, non-atomically, if the file exists.
/// The path be `String` or `Path`.
pub fn write_str<P: AsRef<Path>, S: AsRef<str>>(path: P, data: S) -> Result<()> {
    write(path, data.as_ref().as_bytes())
}


/// Common dirs

pub fn home_dir() -> Option<PathBuf> { dirs_sys::home_dir() }

pub fn config_dir() -> Option<PathBuf> {
    env::var_os("BARDO_CONFIG_HOME").and_then(dirs_sys::is_absolute_path)
                                    .or_else(|| home_dir().map(|h| h.join(".config/bardo"))).map(|h| h.join("gh"))
}

/// Find project dir based on location of Cargo.toml
pub fn project_dir() -> Option<PathBuf> {
    let buf: PathBuf = match env::current_dir().ok() {
        Some(b) => b,
        None => PathBuf::from("/"),
    };
    let path = buf.as_path();

    for ancestor in path.ancestors() {
        let cargo_path = ancestor.join("Cargo.toml");
        if cargo_path.exists() {
            return Some(ancestor.to_path_buf());
        }
    }
    None
}

pub fn write_config_dir() {
    config_dir().map(|buf| std::fs::create_dir_all(buf.as_path()).unwrap_or_else(|why| {
        println!("! {:?}", why.kind());
    }));
}

#[cfg(test)]
mod file_tests {
    use super::*;

    #[test]
    fn test_project_dir() {
        assert_eq!(project_dir(), Some(PathBuf::from("/Users/seka/projects/mttrbit/bardo-github/ghauto-config")));
    }
}
