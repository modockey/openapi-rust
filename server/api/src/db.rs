use chrono::Utc;

use diesel::prelude::*;
use diesel::{insert_into, update};

pub mod model;
use model::schema::ipv4_history::dsl::*;
use model::{Ipv4Record, NewIpV4Record};

use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    if cfg!(test) | cfg!(debug_assertions) {
        dotenv().ok();
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub fn get_effective_records(conn: &PgConnection) -> Vec<Ipv4Record> {
    let ipv4_effective = ipv4_history
        .filter(effective_flg.eq(true))
        .load::<Ipv4Record>(conn)
        .expect("Error loading ipv4_history");
    return ipv4_effective;
}

pub fn insert_record(conn: &PgConnection, address: &str) -> Ipv4Record {
    let now = Utc::now();
    let new_ipv4_record = NewIpV4Record {
        ipv4_address: address.to_string(),
        effective_flg: true,
        created_at: now,
        updated_at: now,
        last_checked_at: now,
    };
    insert_into(ipv4_history)
        .values(new_ipv4_record)
        .get_result(conn)
        .expect("Error saving record")
}

pub fn disable_record(conn: &PgConnection, target_id: &i32) -> Ipv4Record {
    update(ipv4_history.find(target_id))
        .set((effective_flg.eq(false), updated_at.eq(Utc::now())))
        .get_result::<Ipv4Record>(conn)
        .expect(&format!("Error Update Record {}", target_id))
}

pub fn update_last_checked_at(conn: &PgConnection, target_id: &i32) -> Ipv4Record {
    let now = Utc::now();
    update(ipv4_history.find(target_id))
        .set((last_checked_at.eq(now), updated_at.eq(now)))
        .get_result::<Ipv4Record>(conn)
        .expect(&format!("Error Update Record {}", target_id))
}
