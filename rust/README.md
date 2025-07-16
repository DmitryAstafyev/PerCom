## Running the Test

The test can be executed using one of the following commands:

```
docker-compose up --build
```

or

```
sh ./run_test.sh
```

Both commands will result in the same sequence:

- The server will be started using the shared Dockerfile
- Then, a proptest-based test suite will be executed against it

**Note**: Both the server and the test runner use the same Dockerfile.
The runtime behavior is controlled via environment variables:

- `RUST_SERVER_TEST=0` — starts the server
- `RUST_SERVER_TEST=1` — runs the tests