use crate::db;

pub fn get_effective_ipv4_record() -> Result<db::Ipv4Record, String> {
  let conn = db::establish_connection();
  let effective_records = db::get_effective_records(&conn);

  if effective_records.len() == 0 {
    return Err("IPv4 record not found".into());
  }

  if effective_records.len() > 1 {
    return Err("Too many IPv4 records have been found".into());
  }

  return Ok(effective_records[0].clone());
}

pub fn post_ip4_address() -> Result<(), String> {
  let conn = db::establish_connection();
  let effective_records = db::get_effective_records(&conn);

  if effective_records.len() == 0 {
    db::insert_record(&conn, ipv4_address);
    return Ok(());
  }

  if ipv4_address == effective_records[0].ipv4_address {
    db::update_last_checked_at(&conn, &effective_records[0].id);
    return;
  }
  db::disable_record(&conn, &effective_records[0].id);
  db::insert_record(&conn, ipv4_address);
}
