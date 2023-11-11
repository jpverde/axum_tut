# Axum Web App

## Execution of the app

```sh
# Terminal 1 - To run the server.
cargo watch -q -c -w src/ -w .cargo/ -x "run"

# Terminal 2 - To run the quick_dev example.
cargo watch -q -c -w examples/ -w .cargo/ -x "run --example quick_dev"
```

## Starting the DB

```sh
# Start postgresql server docker image:
docker run --rm --name pg -p 5432:5432 \
    -e POSTGRES_PASSWORD=dev_only_pwd \
    postgres:15

# (optional) To have a psql terminal on pg.
# In another terminal (tab) run psql:
docker exec -it -u postgres pg psql
```

## Unit Test (watch)

```sh
cargo watch -q -c -x "test -- --nocapture"

# Specific test with filter
cargo watch -q -c -x "test model::task::tests::test_create -- --nocapture"
```
