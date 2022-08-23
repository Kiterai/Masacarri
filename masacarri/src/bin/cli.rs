use std::env;

use dotenv::dotenv;

use masacarri::comment::*;
use masacarri::db::*;
use masacarri::page::*;

fn add_user(username: &str) {
    
}

fn delete_user(username: &str) {
    
}

fn list_users() {

}

fn list_pages() {

}

fn update_password(username: &str) {

}

fn main() {
    dotenv().ok();
    let args: Vec<String> = env::args().collect();
    
    match args[1].as_str() {
        "adduser" => {
            add_user(args[2].as_str());
        },
        "deluser" => {
            delete_user(args[2].as_str());
        },
        "list" => {
            match args[2].as_str() {
                "user" => list_users(),
                "pages" => list_pages(),
                item => eprintln!("unknown list item: '{}'", item),
            }
        },
        "passwd" => {
            update_password(args[2].as_str());
        },
        cmd => {
            eprintln!("unknown command: '{}'", cmd);
        },
    }

}
