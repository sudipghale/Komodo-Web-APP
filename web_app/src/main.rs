#![feature(proc_macro_hygiene, decl_macro, never_type)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rustc_serialize;
extern crate serde_json;
extern crate rusqlite;

mod komodo;
mod komodorpcutil;
use komodorpcutil::KomodoRPC;

use rusqlite::{params, Connection, Result};
use rustc_serialize::json::Json;
use serde_json::Value as JsonValue;
use std::collections::HashMap;

use rocket::http::{Cookie, Cookies};
use rocket::outcome::IntoOutcome;
use rocket::request::{self, FlashMessage, Form, FromRequest, Request};
use rocket::response::{Flash, Redirect};
use rocket_contrib::templates::Template;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}
#[derive(FromForm)]
struct Signup {
    email: String,
    psw: String,
    pswrepeat: String,
}

#[derive(Debug)]
struct Person {
    id: i32,
    email: String,
    password: String,
}


#[derive(FromForm)]
struct Sent {
    address: String,
    amount: f64,
    comment: String,
}

#[derive(Debug)]
struct User(usize);

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = !;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<User, !> {
        request
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| cookie.value().parse().ok())
            .map(|id| User(id))
            .or_forward(())
    }
}
#[get("/signup")]
fn signup(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }
    Template::render("signup", &context)
}

#[post("/signup", data = "<signup>")]
fn signup_database(signup: Form<Signup>) -> Result<Redirect, Flash<Redirect>>
{
     let conn = Connection::open("db.db").unwrap();
    let me = Person {
        id: 0,
        email: signup.email.to_string(),
        password: signup.psw.to_string(),
    };
    let success = conn.execute(
        "INSERT INTO person (email, password) VALUES (?1, ?2)",
        params![me.email, me.password],
    );
    if success.unwrap() >0  {
        Ok(Redirect::to(uri!(login_page)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(signup)),
            "Invalid username/password.",
        ))
    }
}



#[post("/login", data = "<login>")]
fn login(mut cookies: Cookies, login: Form<Login>) -> Result<Redirect, Flash<Redirect>> {
    let conn = Connection::open("db.db").unwrap();
    let mut stmt = conn.prepare("SELECT id, email, password FROM person").unwrap();
    let person_iter = stmt.query_map(params![], |row| {
        Ok(Person {
            id: row.get(0)?,
            email: row.get(1)?,
            password: row.get(2)?,
        })
    }).unwrap();
    let mut found:bool = false;
    for person in person_iter {
        let temp= person.unwrap();
        if login.username == temp.email && login.password == temp.password {
            found =true;
                }
    }

    if found {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid username/password.",
        ))
    }
}

#[get("/login")]
fn login_user(_user: User) -> Redirect {
    Redirect::to(uri!(user_index))
}

#[get("/login", rank = 2)]
fn login_page(flash: Option<FlashMessage>) -> Template {
    let mut context = HashMap::new();
    if let Some(ref msg) = flash {
        context.insert("flash", msg.msg());
    }

    Template::render("login", &context)
}

#[post("/send_money_post", data = "<send>")]
fn send_money_handler(send: Form<Sent>) -> Result<Redirect, Flash<Redirect>> {
    let someAddress = String::from("127.0.0.1");
    let somePortNum = 13211;
    let someReqMethod = String::from("POST");
    let someUserName = String::from("user629608817");
    let somePassword =
        String::from("pass6375b5f55e1b166f744d7c66b66354217a6d36645d1de5302c8780bca9600cce49");
    let someJSONRPCVer = String::from("1.0");
    let someRPCReqID = String::from("curltest");
    // CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO

    // Create KomodoRPC type (an 'object') with variables
    let someUser = KomodoRPC::new(
        someAddress,
        somePortNum,
        someReqMethod,
        someUserName,
        somePassword,
        someJSONRPCVer,
        someRPCReqID,
    );

    println!("{}", send.address);
    println!("{}", send.amount);
    println!("{}", send.comment);

    komodo::wallet::send_to_address(
        someUser,
        send.address.to_string(),
        send.amount,
        Some(send.comment.to_string()),
        Some("alfonso".to_string()),
        Some(true),
    );

    Ok(Redirect::to(uri!(index)))
    /*if login.username == "Sergio" && login.password == "password" {
        cookies.add_private(Cookie::new("user_id", 1.to_string()));
        Ok(Redirect::to(uri!(index)))
    } else {
        Err(Flash::error(
            Redirect::to(uri!(login_page)),
            "Invalid username/password.",
        ))
    }*/
}

#[post("/logout")]
fn logout(mut cookies: Cookies) -> Flash<Redirect> {
    cookies.remove_private(Cookie::named("user_id"));
    Flash::success(Redirect::to(uri!(login_page)), "Successfully logged out.")
}

#[get("/send")]
fn send_page() -> Template {
    let mut context = "";
    Template::render("send_money_page", &context)
}

#[get("/")]
fn user_index(user: User) -> Template {
    let someAddress = String::from("127.0.0.1");
    let somePortNum = 13211;
    let someReqMethod = String::from("POST");
    let someUserName = String::from("user629608817");
    let somePassword =
        String::from("pass6375b5f55e1b166f744d7c66b66354217a6d36645d1de5302c8780bca9600cce49");
    let someJSONRPCVer = String::from("1.0");
    let someRPCReqID = String::from("curltest");

    // CHECK TO SEE IF THERE IS USERNAME IS EMPTY???? TODO

    // Create KomodoRPC type (an 'object') with variables
    let someUser = KomodoRPC::new(
        someAddress,
        somePortNum,
        someReqMethod,
        someUserName,
        somePassword,
        someJSONRPCVer,
        someRPCReqID,
    );
    let mut context = HashMap::new();
    let requested_amount = komodo::wallet::get_balance(someUser.clone(), None, None).unwrap();
    let json = Json::from_str(&requested_amount).unwrap();
    context.insert("amount", json.find_path(&["result"]).unwrap().to_string());

    let control_info = komodo::control::get_info(someUser.clone()).unwrap();
    let control_result = Json::from_str(&control_info).unwrap();
    let control_json =
        Json::from_str(&control_result.find_path(&["result"]).unwrap().to_string()).unwrap();

    context.insert(
        "name",
        control_json.find_path(&["name"]).unwrap().to_string(),
    );
    context.insert(
        "p2pport",
        control_json.find_path(&["p2pport"]).unwrap().to_string(),
    );

    context.insert(
        "rpcport",
        control_json.find_path(&["rpcport"]).unwrap().to_string(),
    );

    context.insert(
        "balance",
        control_json.find_path(&["balance"]).unwrap().to_string(),
    );
    context.insert(
        "premine",
        control_json.find_path(&["premine"]).unwrap().to_string(),
    );
    context.insert(
        "version",
        control_json.find_path(&["version"]).unwrap().to_string(),
    );

    context.insert(
        "protocolversion",
        control_json
            .find_path(&["protocolversion"])
            .unwrap()
            .to_string(),
    );

    context.insert(
        "walletversion",
        control_json
            .find_path(&["walletversion"])
            .unwrap()
            .to_string(),
    );
    context.insert(
        "blocks",
        control_json.find_path(&["blocks"]).unwrap().to_string(),
    );
    context.insert(
        "connections",
        control_json
            .find_path(&["connections"])
            .unwrap()
            .to_string(),
    );
    context.insert(
        "difficulty",
        control_json.find_path(&["difficulty"]).unwrap().to_string(),
    );
    context.insert(
        "paytxfee",
        control_json.find_path(&["paytxfee"]).unwrap().to_string(),
    );
    context.insert(
        "relayfee",
        control_json.find_path(&["relayfee"]).unwrap().to_string(),
    );

    let something = komodo::wallet::list_address_groupings(someUser).unwrap();
    let res = serde_json::from_str(&something);
    if res.is_ok() {
        let p: JsonValue = res.unwrap();
        context.insert("address1", p["result"][0][0][0].to_string());
        context.insert("balance1", p["result"][0][0][1].to_string());

        context.insert("address2", p["result"][1][0][0].to_string());
        context.insert("balance2", p["result"][1][0][1].to_string());

        context.insert("address3", p["result"][0][1][0].to_string());
        context.insert("balance3", p["result"][0][1][1].to_string());

        println!(
            "{:?} \t\t\t {:?}\n",
            p["result"][2][0][0].as_str(),
            p["result"][2][0][1].as_f64()
        );
    }

    //template to add
    //context.insert("variable_name",control_json.find_path(&["json_variable_name"]).unwrap().to_string(

    Template::render("home_page", &context)
}

#[get("/", rank = 2)]
fn index() -> Template {
    let mut context = "";
    Template::render("landing_page", &context)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().attach(Template::fairing()).mount(
        "/",
        routes![
            index,
            user_index,
            login,
            logout,
            login_user,
            login_page,
            send_page,
            send_money_handler,
            signup,
            signup_database
        ],
    )
}

fn main() {
    rocket().launch();
}
