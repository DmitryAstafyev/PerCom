docker build -t rust-server-archlinux .
docker run -e RUST_SERVER_TEST=1 rust-server-archlinux