#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn file_exists_home(){
        let t = super::file_exists(super::get_home().unwrap().as_path());
        assert_eq!(t, true);
    }
    #[test]
    fn file_exists_void(){
        let p = std::path::PathBuf::from("/home/thisdoesnotexist");
        let t = super::file_exists(&p.as_path());
        assert_eq!(t, false);
    }
}

use std::env;
use std::path::{Path,PathBuf};
use std::fs::File;
use std::io::Read;

/// Returns the home directory of the user as a String.
///
/// # Example
///
/// ```
/// extern crate simpleio as sio;
/// println!("{:?}", sio::get_home_string());
/// ```
/// It returns the same as:
/// ```
/// echo "$HOME"
/// ```
pub fn get_home_string() -> Option<String>{
    match env::var("HOME"){
        Ok(val) => Option::Some(val),
        Err(_e) => Option::None,
    }
}

/// Returns the home directory of the user as a std::path::PathBuf.
///
/// # Example
///
/// ```
/// extern crate simpleio as sio;
/// println!("{:?}", sio::get_home());
/// ```
/// It returns the same as:
/// ```
/// echo "$HOME"
/// ```
pub fn get_home() -> Option<PathBuf>{
    match env::var("HOME"){
        Ok(val) => Option::Some(PathBuf::from(val)),
        Err(_e) => Option::None,
    }
}

/// Returns the config directory.
/// It is just home/.config.
/// All config files the application uses should be in home/.config/application-name.
/// They should NOT be writen in the home directory as it clutters up the home dir and the .config was made for these files.
/// However, many applications do this wrong.
/// Just type ```la``` instead of ```ls``` inside your home dir to see evidence of that.
///
/// # Example
///
/// ```
/// extern crate simpleio as sio;
/// println!("{:?}", sio::get_config());
/// ```
pub fn get_config() -> Option<PathBuf>{
    let mut home = get_home()?;
    home.push(".config");
    Option::Some(home)
}

/// Returns true if the file exists, false if not.
/// Directories are also files on unix.
///
/// # Example
///
/// ```
/// extern crate simpleio as sio;
/// //asuming home is set and will return a value
/// println!("{}", sio::file_exists(sio::get_home().unwrap().as_path()));
/// ```
pub fn file_exists(path: &Path) -> bool{
    let metadata = std::fs::metadata(path);
    match metadata{
        Ok(_x) => true,
        Err(_y) => false,
    }
}

/// Enum for possible outcomes when creating a directory
pub enum DirStatus { Exists, Created, Error, }

/// Creates the directory if it does not already exist.
///
/// # Example
///
/// ```
/// extern crate simpleio as sio;
/// match sio::create_dir(sio::get_home().unwrap().as_path()){
///     sio::DirStatus::Exists => println!("Home exists."),
///     sio::DirStatus::Created => println!("Home created."),
///     sio::DirStatus::Error => println!("Home could not be created."),
/// }
/// ```
pub fn create_dir(path: &Path) -> DirStatus{
    let metadata = std::fs::metadata(path);
    if metadata.is_ok(){
        return DirStatus::Exists;
    }
    let res = std::fs::create_dir_all(path);
    if res.is_ok(){
        return DirStatus::Created;
    }
    return DirStatus::Error;
}

/// Reads file to the end into Vec<u8>
pub fn read_file_into_buffer(file_path: &str) -> Result<Vec<u8>, std::io::Error>{
    let mut buffer = Vec::new();
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => { return Err(e); }
    };
    match file.read_to_end(&mut buffer){
        Ok(_) => Ok(buffer),
        Err(e) => Err(e),
    }
}

