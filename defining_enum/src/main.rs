enum IpAddrKind<T> {
    V4(T),
    V6(T),
    None,
}

fn ip_type<T>(ip: &IpAddrKind<T>) {
    let _localhost = String::from("127.0.0.1");
    if let IpAddrKind::V4(_localhost) = ip {
        println!("Localhost identificado");
    } else{
        println!("Não é localhost");
        match ip{
            IpAddrKind::V4(_t) => {
                println!("ip versao 4");
            },
            IpAddrKind::V6(_t) => {
                println!("ip versao 6");
            },
            IpAddrKind::None=> {println!("None found")},
        }
    }
}

fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));
    let none: IpAddrKind<i32> = IpAddrKind::None;
    
    ip_type(&four);
    ip_type(&six);
    ip_type(&none);    
}
