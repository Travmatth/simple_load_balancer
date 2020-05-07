use std::{env, error::Error, fs, net};

fn load_opts(opts: Vec<String>) -> Result<(i64, Vec<net::IpAddr>), Box<dyn Error + 'static>> {
    let port = opts[1].parse()?;
    let data = fs::read_to_string(&opts[2])?;
    let mut servers = Vec::new();

    for server in data.lines().collect::<Vec<_>>() {
        servers.push(server.parse::<net::IpAddr>()?);
    }
    Ok((port, servers))
}

fn main() {
    if env::args().len() != 3 {
        println!("Usage: ./lb <port> <serve_list.txt>");
        return;
    }
    let args = env::args().collect::<Vec<String>>();
    let (_, servers) = load_opts(args).unwrap();
    println!("{:?}", servers);
}
