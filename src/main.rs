extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate persistent;
extern crate router;
extern crate serde_json;
extern crate longgu;
extern crate r2d2_diesel;

use dotenv::dotenv;
use iron::prelude::*;
use iron::typemap::Key;
use iron::status;
use persistent::Read;
use router::Router;
use diesel::*;

use longgu::connection::{Account, Pool};

struct PoolKey;

impl Key for PoolKey {
    type Value = Pool;
}

fn main() {
    dotenv().ok();
    let pool = longgu::connection::establish_connection_pool();

    let mut router = Router::new();
    router.get("/accounts", handler, "index");

    let mut chain = Chain::new(router);
    chain.link_before(Read::<PoolKey>::one(pool));

    Iron::new(chain).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let connection = req.get::<Read<PoolKey>>().unwrap().get().unwrap();
        let results = {
            use longgu::schema::accounts::dsl::*;
            accounts.load::<Account>(&*connection)
                .expect("Error loading posts")
        };
        Ok(Response::with((status::Ok, serde_json::to_string(&results).unwrap())))
    }
}
