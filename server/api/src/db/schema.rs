table! {
    ipv4_history (id) {
        id -> Int4,
        ipv4_address -> Varchar,
        effective_flg -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        last_checked_at -> Timestamptz,
    }
}
