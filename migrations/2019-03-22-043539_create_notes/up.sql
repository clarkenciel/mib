-- Your SQL goes here
create table notes (
  id serial primary key,
  note_id uuid not null unique,
  content text not null,
  publication_date timestamptz not null
);
