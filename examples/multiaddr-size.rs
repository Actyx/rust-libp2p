use multiaddr::Multiaddr;

fn main() {
    let mut t: Vec<Multiaddr> = Vec::new();
    for _ in 0..1000000 {
        t.push("/ip4/127.0.0.1/tcp/1234".parse().unwrap());
    }
}