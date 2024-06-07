use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::BufRead;
use std::io::Write;


#[derive(Debug, Serialize, Deserialize)]
pub struct PlatformInfo{
    pub platform: String,
    pub username: String,
    pub password: String,
}

impl PlatformInfo{
    pub fn new(platform: String, username: String, password: String) -> Self {
        PlatformInfo{
            platform,
            username,
            password,
        }
    }

    pub fn from_json(json_string: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json_string)
    }
    
    #[allow(dead_code)]
    pub fn from_user_input() -> Self {
        println!("Enter platform : ");
        let mut platform: String = String::new();
        io::stdin().read_line(&mut platform).unwrap();

        println!("Enter username : ");
        let mut username: String = String::new();
        io::stdin().read_line(&mut username).unwrap();

        println!("Enter password : ");
        let mut password: String = String::new();
        io::stdin().read_line(&mut password).unwrap();

        ServiceInfo::new(
            platform.trim().to_string(),
            username.trim().to_string(),
            password.trim().to_string()
        )
    }

    fn to_json(&self) -> String {
        serde_json::to_string(&self).expect("Failed to serialize to JSON")
    }

    pub fn write_to_file(&self) {
        let json: String = format!("{}\n", self.to_json());

        match OpenOptions::new().create(true).append(true).open("passwords.json"){
            Ok(mut file) => {
                if let Err(e) = file.write_all(json.as_bytes()){
                    eprintln!("Error writing file : {e}")
                } else {
                    println!("Successfull")
                }
            }
            Err(e) => eprintln!("Error opening file: {e}"),
        }

    }
}

pub fn read_passwords_from_file() -> Result<Vec<PlatformInfo>, io::Error> {
    let file: File = File::open("passwords.json")?;
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut services = Vec::new();

    for line in reader.lines(){
        if let Ok(json_string) = line{
            if let Ok(service_info) = PlatformInfo::from_json(&json_string){
                services.push(service_info);
            } else {
                eprintln!("Error read ..... ")
            }
        } else {
            eprintln!("Erroor read 2 ...")
        }
    }
    Ok(services)
}

pub fn prompt(prompt : &str) -> String {
    print!("{prompt}");
    io::stdout().flush().unwrap();
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
