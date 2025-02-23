-- Add migration script here
create table subscriptions(
    id uuid not null primary key,
    email text not null unique,
    name text not null,
    subscribed_at timestamptz not null
)