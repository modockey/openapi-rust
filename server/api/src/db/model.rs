use chrono::{DateTime, Utc};

pub mod schema;
use schema::ipv4_history;

#[allow(dead_code)]
#[derive(Clone, Queryable)]
pub struct Ipv4Record {
    pub id: i32,
    pub ipv4_address: String,
    effective_flg: bool,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    pub last_checked_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "ipv4_history"]
pub struct NewIpV4Record {
    pub ipv4_address: String,
    pub effective_flg: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_checked_at: DateTime<Utc>,
}
