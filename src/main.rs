use std::env;
//#![no_std]
fn main() {
    //Get profile
    let profile_key = "profile";
    let profile = "";
    match env::var(profile_key) {
        Ok(val) => {
            let profile = &String::from(val);
            println!("Value is {}", profile);
        },
        Err(e) => println!("couldn't interpret {}: {}", profile_key, e),
    }

    //Create Target path for loader.o
    let target_path: String = "-o target/".to_owned();
    let profile_path:String  = String::from(profile).to_owned();
    let output_file: String = "loader.o".to_owned();

    let loader_output = [target_path, profile_path, output_file].join("");

    println!("Value is {}", loader_output);

    if Ok("dev".to_owned()) == env::var("PROFILE") {
        panic!("I'm only panicking in release mode")
    }
}