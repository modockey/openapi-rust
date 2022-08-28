use crate::db;
use db::*;

use regex::Regex;

pub fn get_effective_ipv4_record() -> Result<db::model::Ipv4Record, String> {
    let conn = establish_connection();
    let effective_records = get_effective_records(&conn);

    if effective_records.len() == 0 {
        return Err("IPv4 record not found".into());
    }

    if effective_records.len() > 1 {
        return Err("Too many IPv4 records have been found".into());
    }

    return Ok(effective_records[0].clone());
}

pub fn post_ip4_address(ipv4_address: &str) -> Result<(), String> {
    let conn = establish_connection();
    let effective_records = get_effective_records(&conn);

    if effective_records.len() == 0 {
        insert_record(&conn, ipv4_address);
        return Ok(());
    }

    if ipv4_address == effective_records[0].ipv4_address {
        update_last_checked_at(&conn, &effective_records[0].id);
        return Ok(());
    }

    disable_record(&conn, &effective_records[0].id);
    insert_record(&conn, ipv4_address);
    return Ok(());
}

pub fn is_ipv4(text: &str) -> bool {
    let re = Regex::new(
        r"^((25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])\.){3}(25[0-5]|2[0-4][0-9]|1[0-9][0-9]|[1-9]?[0-9])$",
    ).unwrap();
    return re.is_match(text);
}
