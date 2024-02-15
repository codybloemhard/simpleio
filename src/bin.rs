use simpleio as sio;

pub fn main(){
    test_get_home_string();
    test_get_home();
    test_file_exist();
    test_get_config();
}

fn test_get_home_string(){
    println!("{:?}", sio::get_home_string());
}

fn test_get_home(){
    println!("{:?}", sio::get_home());
}

fn test_file_exist(){
    println!("{}", sio::file_exists(sio::get_home().unwrap().as_path()));
}

#[allow(dead_code)]
fn test_create_dir(){
    match sio::create_dir(sio::get_home().unwrap().as_path()){
        sio::DirStatus::Exists => println!("Home exists."),
        sio::DirStatus::Created => println!("Home created."),
        sio::DirStatus::Error => println!("Home could not be created."),
    }
}

fn test_get_config(){
    println!("{:?}", sio::get_config());
}

