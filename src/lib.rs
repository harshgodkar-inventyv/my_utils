pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}


// pub mod register {
//     include!(concat!(env!("OUT_DIR"), "/register.rs"));
// }
// pub mod login {
//     include!(concat!(env!("OUT_DIR"), "/login.rs"));
// }
// pub mod crm {
//     include!(concat!(env!("OUT_DIR"), "/crm.rs"));
// }

pub mod myapp {
    include!(concat!(env!("OUT_DIR"), "/myapp.rs"));
}

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
}