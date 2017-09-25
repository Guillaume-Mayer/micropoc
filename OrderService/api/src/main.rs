extern crate iron;
extern crate router;
extern crate postgres;
extern crate bodyparser;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use router::Router;
use std::error::Error;
use std::env;
use std::net::{Ipv4Addr, SocketAddrV4};
use postgres::{Connection, TlsMode};

struct Client {
    id: i32,
    name: String,
    email: String,
    comuna: String,
    direccion: String,
    depto: String
}

#[derive(Debug)]
struct Order {
    id: i32,
    client: i32,
    product: String,
    qty: i32,
    status: String
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

    fn get_order(req: &mut Request) -> IronResult<Response> {
        let ref id = req.extensions.get::<Router>().unwrap().find("id").unwrap();
        let content_type : Mime = "application/json".parse().unwrap();
        Ok(Response::with((content_type, status::Ok, format!("{{\"order\": \"{}\"}}", id))))
    }

    fn post_order(req: &mut Request) -> IronResult<Response> {
        let json_body = req.get::<bodyparser::Json>();
        match json_body {
            Ok(Some(json_body)) => {
                let product = json_body["product"];
                let client = json_body["client"];
                let qty = json_body["qty"];
                let o = Order {
                    id: 0,
                    client: client.,
                    product: product,
                    qty: qty,
                    status: "Initial".to_owned(),
                };
                println!("order: {:?}", o);
            },
            Ok(None) => println!("No body"),
            Err(err) => println!("Error: {:?}", err)
        }
        /*let conn = Connection::connect("postgresql://orders:order123@localhost:5432", TlsMode::None).unwrap();
         conn.execute(
            "INSERT INTO client (name, email, comuna, direccion, depto) VALUES ($1, $2, $3, $4, $5)",
            &[&cli.name, &cli.email, &cli.comuna, &cli.direccion, &cli.depto]
            ).unwrap();*/
        let content_type : Mime = "application/json".parse().unwrap();
        Ok(Response::with((content_type, status::Ok, format!("{{\"order\": \"newID\"}}"))))
    }

    // Server
    match Iron::new(router).http(SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), port)) {
        Ok(_) => println!("Listening on {}", port),
        Err(e) => println!("Error: {}", e.description()),
    };

}