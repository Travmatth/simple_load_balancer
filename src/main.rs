use std::{
    env,
    error::Error,
    fs,
    net::{Ipv4Addr, SocketAddrV4},
};

fn load_opts(opts: Vec<String>) -> Result<(SocketAddrV4, Vec<Ipv4Addr>), Box<dyn Error + 'static>> {
    let port = opts[1].parse()?;
    let socket_addr = SocketAddrV4::new(Ipv4Addr::LOCALHOST, port);
    let contents = fs::read_to_string(&opts[2])?;
    let mut servers = Vec::new();

    for server in contents.lines().collect::<Vec<_>>() {
        servers.push(server.parse::<Ipv4Addr>()?);
    }
    Ok((socket_addr, servers))
}

fn main() {
    if env::args().len() != 3 {
        println!("Usage: ./lb <port> <serve_list.txt>");
        return;
    }
    let args = env::args().collect::<Vec<String>>();
    let (socket, servers) = load_opts(args).unwrap();
    println!("{:?} {:?}", socket, servers);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_vec_string(arr: Vec<&str>) -> Vec<String> {
        arr.into_iter().map(|s| s.to_owned()).collect()
    }

    #[test]
    fn load_opts_rejects_invalid_port() {
        let opts = to_vec_string(vec!["a", "b", "c"]);
        if let Ok(_) = load_opts(opts) {
            panic!("load_opts should reject invalid port")
        }
    }

    #[test]
    fn load_opts_rejects_invalid_filename() {
        let opts = to_vec_string(vec!["lb", "80", "b"]);
        if let Ok(_) = load_opts(opts) {
            panic!("load_opts should reject invalid port")
        }
    }

    #[test]
    fn test_load_opts() {
        let opts = to_vec_string(vec!["lb", "80", "tests/list.txt"]);
        if let Ok((s, _)) = load_opts(opts) {
            assert_eq!(s, SocketAddrV4::new(Ipv4Addr::LOCALHOST, 80));
        }
    }
}
