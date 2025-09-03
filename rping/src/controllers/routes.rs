#[get("/")]
pub fn index() -> String {
    "Hello, world!".to_string()
}
