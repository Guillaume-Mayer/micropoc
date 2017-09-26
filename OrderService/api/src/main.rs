extern crate iron;
extern crate router;
extern crate postgres;
extern crate bodyparser;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::error::Error;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use postgres::{Connection, TlsMode};

#[allow(dead_code)]
struct Client {
    id: i32,
    name: String,
    email: String,
    comuna: String,
    direccion: String,
    depto: String
}

#[derive(Debug, Clone, Deserialize)]
struct Order {
    client: i32,
    product: String,
    qty: i32,
}

fn main() {

    // Get port from args
    let mut args = env::args();
    args.next();
    let port : u16 = args.next().unwrap_or(String::from("3002")).parse().unwrap_or(3002);

    // Routes
    let mut router = Router::new();
    router.get("/order/:id", get_order, "get_order");
    router.post("/order", post_order, "post_order");

    // Get Order Handler
    fn get_order(req: &mut Request) -> IronResult<Response> {
        let content_type : Mime = "application/json".parse().unwrap();
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        Ok(Response::with((content_type, status::Ok, format!("{{\"order\": \"{}\"}}", id))))
    }

    // Post Order Handler
    fn post_order(req: &mut Request) -> IronResult<Response> {
        let content_type : Mime = "application/json".parse().unwrap();
        let body = req.get::<bodyparser::Struct<Order>>();
        match body {
            Ok(Some(body)) => {
                insert_order(&body);
            },
            Ok(None) => println!("No body"),
            Err(err) => println!("Error: {:?}", err)
        };
        Ok(Response::with((content_type, status::Ok, format!("{{\"order\": \"newID\"}}"))))
    }

    // Insert an order in DB
    fn insert_order(order: &Order) {
        let conn = Connection::connect("postgresql://orders:order123@localhost:5432", TlsMode::None).unwrap();
        match conn.execute(
            "INSERT INTO orders (product, qty, client) VALUES ($1, $2, $3)",
            &[&order.product, &order.qty, &order.client]
            ) {
                Ok(status) => println!("Insert OK [status={}]", status),
                Err(err) => println!("Error: {:?}", err)
            };
    }

    // Server
    match Iron::new(router).http(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)) {
        Ok(_) => println!("Listening on {}", port),
        Err(e) => println!("Error: {}", e.description()),
    };

}