#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
extern crate rocket_contrib;

#[cfg(test)]
mod tests;

use rocket::response::Redirect;
use rocket::Request;
use rocket_contrib::templates::{handlebars, Template};

use handlebars::{Context, Handlebars, Helper, HelperResult, JsonRender, Output, RenderContext};

#[derive(Serialize)]
struct TemplateContext {
    title: &'static str,
    name: Option<String>,
    // This key tells handlebars which template is the parent.
    parent: &'static str,
}


#[get("/")]
fn index() -> Template {
    Template:: render(
        "home",
        &TemplateContext {
            title: "home",
            name: None,
            parent: "layout",
        },
    )
}
#[get("/signin")]
fn signin() -> Template {
    Template::render(
        "signin",
        &TemplateContext {
            title: "signin",
            name: None,
            parent: "layout",
        },
    )
}

#[get("/signup")]
fn signup() -> Template {
    Template::render(
        "signup",
        &TemplateContext {
            title: "signup",
            name: None,
            parent: "layout",
        },
    )
}
#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = std::collections::HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}

fn wow_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut Output,
) -> HelperResult {
    if let Some(param) = h.param(0) {
        out.write("<b><i>")?;
        out.write(&param.value().render())?;
        out.write("</b></i>")?;
    }

    Ok(())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index, signin, signup])
        .register(catchers![not_found])
        .attach(Template::custom(|engines| {
            engines
                .handlebars
                .register_helper("wow", Box::new(wow_helper));
        }))
}

fn main() {
    rocket().launch();
}
