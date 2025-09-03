use std::collections::HashMap;

use rocket::futures::lock::Mutex;

pub type HostMap = Mutex<HashMap<String, String>>;
