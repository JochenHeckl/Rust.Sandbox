enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    let adress_list = [home, loopback];

    for adress in &adress_list {
        match adress {
            IpAddr::V4(a, b, c, d) => println!("V4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(ip) => println!("V6: \"{}\"", ip),
        }
    }
}