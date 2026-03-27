use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Result;
use std::fs;
use std::io;
#[derive(Serialize, Deserialize, Debug)]
struct Account {
    username: String,
    password: String,
}

fn create_account() -> Result<()> {
    println!("Name OF The File That's You Wanna STORE in");
    let mut filename = String::new(); // filename thats has infos
    io::stdin().read_line(&mut filename).unwrap();
    println!("Enter Your Username: ");
    let mut input1 = String::new(); //username
    io::stdin().read_line(&mut input1).unwrap();
    println!("Enter Your Password: ");
    let mut input2 = String::new(); //password
    io::stdin().read_line(&mut input2).unwrap();
    let filedata = Account {
        username: input1.trim().to_string(),
        password: input2.trim().to_string(),
    };
    let convert_to_json_file = serde_json::to_string_pretty(&filedata)?;
    fs::write(filename.trim().to_owned() + ".json", convert_to_json_file).unwrap();
    Ok(())
}
fn login() {
    println!("Enter The name of the file");
    let mut filename = String::new();
    let mut input1 = String::new();
    let mut input2 = String::new();
    io::stdin().read_line(&mut filename).unwrap();
    println!("Enter Username :");
    io::stdin().read_line(&mut input1).unwrap();
    println!("Enter Password :");
    io::stdin().read_line(&mut input2).unwrap();
    let filedata = fs::read_to_string(filename.trim().to_owned()+ ".json").unwrap();
    let translate_to_string: Account = serde_json::from_str(&filedata).unwrap();
    if input2.trim() == translate_to_string.password {
        println!("Login Successfully Completed!") ;
        println!("Welcome {}",translate_to_string.username);
    } else {
        println!("Wrong Passcode ! Shutting Down"); 
    }    
}
fn main() {
    println!("Enter L to Login or C To Signin");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if input.trim() == "C" {
        create_account().unwrap();
    } else if input.trim() == "L" {
        login();
    } else {
        println!("No Input Found !")
    }

}