use std::path::Path;
use std::ffi::OsStr;

mod walk;

pub use self::walk::walk_dirs;
pub use self::walk::walk_files;

/// Gets the filename of a given path
pub fn fname(path : &Path) -> &str {
	path.file_name().unwrap_or(OsStr::new("")).to_str().unwrap_or("")
}
