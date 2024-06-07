mod reader;

use std::io;

use crate::reader::prompt;
use crate::reader::read_passwords_from_file;
use reader::PlatformInfo;

fn clear() {
    print!("{}[2J", 27 as char)
}

fn main() {
    clear();
    let header: &str = r#"
     /$$$$$$$  /$$$$$$$$ /$$$$$$$  /$$$$$$        /$$$$$$   /$$$$$$  /$$$$$$$  /$$$$$$ /$$   /$$  /$$$$$$ 
    | $$__  $$| $$_____/| $$__  $$|_  $$_/       /$$__  $$ /$$__  $$| $$__  $$|_  $$_/| $$$ | $$ /$$__  $$
    | $$  \ $$| $$      | $$  \ $$  | $$        | $$  \__/| $$  \ $$| $$  \ $$  | $$  | $$$$| $$| $$  \__/
    | $$$$$$$/| $$$$$   | $$  | $$  | $$        | $$      | $$  | $$| $$  | $$  | $$  | $$ $$ $$| $$ /$$$$
    | $$__  $$| $$__/   | $$  | $$  | $$        | $$      | $$  | $$| $$  | $$  | $$  | $$  $$$$| $$|_  $$
    | $$  \ $$| $$      | $$  | $$  | $$        | $$    $$| $$  | $$| $$  | $$  | $$  | $$\  $$$| $$  \ $$
    | $$  | $$| $$$$$$$$| $$$$$$$/ /$$$$$$      |  $$$$$$/|  $$$$$$/| $$$$$$$/ /$$$$$$| $$ \  $$|  $$$$$$/
    |__/  |__/|________/|_______/ |______/       \______/  \______/ |_______/ |______/|__/  \__/ \______/                                                                                              
    "#;

    println!("{header}");

    loop {
        println!("Manager Menu");
        println!("1. Add Platform");
        println!("2. List Accounts");
        println!("3. Search Accounts");
        println!("4. Exit");

        let mut pick: String = String::new();
        io::stdin().read_line(&mut pick).unwrap();

        match pick.trim(){
            "1" => {
                clear();
                let add: PlatformInfo = PlatformInfo::new(
                    prompt("Platform: "), 
                    prompt("Username: "),
                    prompt("Password: "),
                );
                println!("User added succesfully");
                add.write_to_file();
            }
            "2" => {
                clear();
                let users: Vec<PlatformInfo> = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error user list : {err}");
                    Vec::new()
                });
                for item in &users{
                    println!(
                        "Platfrom = {}
                        - Username : {}
                        - Password : {}
                        ", item.platform, item.username, item.password
                    )
                }
            }
            "3" => {
                clear();
                let users: Vec<PlatformInfo> = read_passwords_from_file().unwrap_or_else(|err| {
                    eprintln!("Error user list : {err}");
                    Vec::new()
                });
                let search: String = prompt("Search :");
                for item in &users{
                    if item.platform.as_str() == search.as_str(){
                        println!(
                            "Platfrom = {}
                            - Username : {}
                            - Password : {}
                            ", item.platform, item.username, item.password
                        )
                    }
                }
            }
            "4" => {
                clear();
                println!("See you later @@@");
                break;
            }
            _ => println!("Invalid number. Please try again...")
        }
    }
}
