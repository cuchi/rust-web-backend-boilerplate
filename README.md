# Rust Backend Boilerplate

This little project is a boilerplate that features:

- Rust
- Rocket
- Diesel (with PostgreSQL)

## What it does?

It will just be same old Todo CRUD with user authentication. This is intended
to be used as a starting point for a new project in this stack.

## Running

Just ensure that PostgreSQL is running with the credentials defined in the
`.env` and the `Rocket.toml` files, then run `cargo run`.

## TODO

- [ ] Add authentication
- [ ] Add tests
- [ ] Add a `docker-compose.yml`
- [ ] Implement the small feature set of the CRUD
