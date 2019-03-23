#![feature(plugin, custom_derive, const_fn, decl_macro, extern_prelude)]
#![plugin(rocket_codegen)]
extern crate rocket;
extern crate rocket_contrib;
fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
}
#[get("/")]
fn index<'a>() -> &'a str {
    "Hello!"
}
fn main() {
    rocket().launch();
}