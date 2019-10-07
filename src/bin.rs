extern crate simpleio as sio;

pub fn main(){
    println!("haha yes");
    test_get_config();
}

fn test_get_home_string(){
    extern crate simpleio as sio;
    println!("{:?}", sio::get_home_string());
}

fn test_get_home(){
    extern crate simpleio as sio;
    println!("{:?}", sio::get_home());
}

fn test_file_exist(){
    extern crate simpleio as sio;
    println!("{}", sio::file_exists(sio::get_home().unwrap().as_path()));
}

fn test_create_dir(){
    extern crate simpleio as sio;
    match sio::create_dir(sio::get_home().unwrap().as_path()){
        sio::DirStatus::Exists => println!("Home exists."),
        sio::DirStatus::Created => println!("Home created."),
        sio::DirStatus::Error => println!("Home could not be created."),
    }
}

fn test_get_config(){
    extern crate simpleio as sio;
    println!("{:?}", sio::get_config());
}
