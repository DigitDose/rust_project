// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
//}

fn main() {
    enum IpAddr {
        V4(u8, u8, u8, u8), // alternatywa dla struktury
        V6(String),
    }
    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
