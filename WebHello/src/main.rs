use std::net::TcpListener;

fn main() {
    let listener = TcpListene::bind("127.0.0.1:7878").unwrap();

    for tream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
    }
}
