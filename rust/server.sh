docker build -t rust-server-archlinux .
docker run -e RUST_SERVER_TEST=0 -p 8080:8080 rust-server-archlinux