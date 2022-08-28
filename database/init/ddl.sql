drop table if exists ipv4_history;

create table ipv4_history (
  id serial,
  ipv4_address varchar(15) not null,
  effective_flg boolean not null,
  created_at timestamptz not null,
  updated_at timestamptz not null,
  last_checked_at timestamptz not null,
  PRIMARY KEY (id)
);