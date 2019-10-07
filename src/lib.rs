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

/// Returns the home directory of the user as a String.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// println!("{:?}", tbl::get_home_string());
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
/// use term_basics_linux as tbl;
/// println!("{:?}", tbl::get_home());
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
/// use term_basics_linux as tbl;
/// println!("{:?}", tbl::get_config());
/// ```
pub fn get_config() -> Option<PathBuf>{
    let home = get_home();
    if home.is_none(){
        return Option::None;
    }
    let mut home = home.unwrap();
    home.push(".config");
    return Option::Some(home);
}

/// Returns true if the file exists, false if not.
/// Directories are also files on unix.
/// 
/// # Example
/// 
/// ```
/// use term_basics_linux as tbl;
/// //asuming home is set and will return a value
/// tbl::println(tbl::file_exists(tbl::get_home().unwrap().as_path()));
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
/// use term_basics_linux as tbl;
/// match tbl::create_dir(tbl::get_home().unwrap().as_path()){
///     tbl::DirStatus::Exists => tbl::println("Home exists."),
///     tbl::DirStatus::Created => tbl::println("Home created."),
///     tbl::DirStatus::Error => tbl::println("Home could not be created."),
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
