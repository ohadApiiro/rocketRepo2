mod handlers;

#[macro_use] extern crate rocket;

#[launch]
fn rocket() -> _ {

    rocket::build().mount("hello", routes![handlers::world, handlers::muchachos])
}
