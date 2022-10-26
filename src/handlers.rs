extern crate rocket;

#[get("/matokies")]
pub fn world() -> &'static str {
    "Hello matokies"
}

#[post("/muchachos")]
pub fn muchachos() -> &'static str {
    "gringo"
}