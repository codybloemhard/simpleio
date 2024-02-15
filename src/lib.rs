use std::env;
use std::path::{ Path, PathBuf};
use std::fs::File;
use std::io::Read;
use std::env::VarError;
use std::io;
use std::io::BufRead;

/// Returns the home directory of the user as a String.
///
/// # Example
///
/// ```
/// use simpleio as sio;
/// println!("{:?}", sio::get_home_string());
/// ```
/// It returns the same as:
/// ```sh
/// echo "$HOME"
/// ```
pub fn get_home_string() -> Result<String, VarError>{
    env::var("HOME")
}

/// Returns the home directory of the user as a std::path::PathBuf.
///
/// # Example
///
/// ```
/// use simpleio as sio;
/// println!("{:?}", sio::get_home());
/// ```
/// It returns the same as:
/// ```sh
/// echo "$HOME"
/// ```
pub fn get_home() -> Result<PathBuf, VarError>{
    match env::var("HOME"){
        Ok(val) => Ok(PathBuf::from(val)),
        Err(e) => Err(e)
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
/// use simpleio as sio;
/// println!("{:?}", sio::get_config());
/// ```
pub fn get_config() -> Result<PathBuf, VarError>{
    let mut home = get_home()?;
    home.push(".config");
    Ok(home)
}

/// Returns true if the file exists, false if not.
/// Directories are also files on unix.
///
/// # Example
///
/// ```
/// use simpleio as sio;
/// //asuming home is set and will return a value
/// println!("{}", sio::file_exists(sio::get_home().unwrap().as_path()));
/// ```
pub fn file_exists(path: &Path) -> bool{
    std::fs::metadata(path).is_ok()
}

/// Enum for possible outcomes when creating a directory
pub enum DirStatus { Exists, Created, Error }

/// Creates the directory if it does not already exist.
///
/// # Example
///
/// ```
/// use simpleio as sio;
/// match sio::create_dir(sio::get_home().unwrap().as_path()){
///     sio::DirStatus::Exists => println!("Home exists."),
///     sio::DirStatus::Created => println!("Home created."),
///     sio::DirStatus::Error => println!("Home could not be created."),
/// }
/// ```
pub fn create_dir(path: &Path) -> DirStatus{
    if file_exists(path){
        return DirStatus::Exists;
    }
    let res = std::fs::create_dir_all(path);
    if res.is_ok(){
        DirStatus::Created
    } else {
        DirStatus::Error
    }
}

/// Reads file to the end into Vec<u8>.
pub fn read_file_into_buffer<P>(file_path: P) -> Result<Vec<u8>, std::io::Error>
    where P: AsRef<Path>
{
    let mut buffer = Vec::new();
    let mut file = File::open(file_path)?;
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Reads file to the end into String.
pub fn read_file_into_string<P>(file_path: P) -> Result<String, std::io::Error>
    where P: AsRef<Path>
{
    let mut string = String::new();
    let mut file = File::open(file_path)?;
    file.read_to_string(&mut string)?;
    Ok(string)
}

/// Reads lines from file. Returns Lines iterator.
pub fn read_lines_raw<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

/// Reads lines from file. Returns lines in vector.
pub fn read_lines<P>(filename: P) -> Vec<String>
    where P: AsRef<Path>
{
    let mut res = Vec::new();
    if let Ok(ls) = read_lines_raw(filename) {
        for l in ls.map_while(Result::ok) {
            res.push(l);
        }
    }
    res
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn file_exists_home(){
        let t = super::file_exists(super::get_home().unwrap().as_path());
        assert!(t);
    }

    #[test]
    fn file_exists_void(){
        let p = std::path::PathBuf::from("/home/thisdoesnotexist");
        let t = super::file_exists(p.as_path());
        assert!(!t);
    }
}

