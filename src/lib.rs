pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


pub mod register {
    include!(concat!(env!("OUT_DIR"), "/register.rs"));
}