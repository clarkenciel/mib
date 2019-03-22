-- Your SQL goes here
alter table notes add column current_location point not null default '(0,0)';
