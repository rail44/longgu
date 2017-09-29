extern crate diesel;
extern crate dotenv;
extern crate longgu;

use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;

use longgu::connection;
use connection::{NewInstance, NewAccount, establish_connection};

fn main() {
    dotenv().ok();
    let connection = establish_connection();
    connection.transaction::<_, Error, _>(|| {

        let self_instance_id = {
            use longgu::schema::instances::dsl::*;
            let self_instance = NewInstance {
                domain: "example.com".to_string(),
                is_myself: true,
            };
            diesel::insert(&self_instance)
                .into(instances)
                .returning(id)
                .execute(&connection)
                .unwrap()
        };

        {
            use longgu::schema::accounts::dsl::*;
            let new_account = NewAccount {
                user_name: "john".to_string(),
                display_name: "John Doe".to_string(),
                instance_id: self_instance_id as i32,
            };

            diesel::insert(&new_account)
                .into(accounts)
                .execute(&connection)
                .unwrap();
        }
        Ok(())
    }).unwrap();
}
