fn main() {
    println!("Hello, world!");

    let config = swarm::parse_config();
    println!("{:#?}", config);
}
