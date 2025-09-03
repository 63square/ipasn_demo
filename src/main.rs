use prost::Message;

mod protos {
    include!("protos/_.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;

    socket.connect("127.0.0.1:36841").await?;

    let request = protos::IpLookupRequest {
        ips: vec![
            String::from("146.70.117.189"),
            String::from("66.70.181.240"),
            String::from("185.107.56.210"),
            String::from("162.247.74.200"),
            String::from("172.67.0.0"),
            String::from("103.21.244.0"),
            String::from("104.244.42.0"),
            String::from("216.58.192.0"),
            String::from("185.199.111.153"),
            String::from("104.18.28.42"),
            String::from("199.232.41.10"),
            String::from("192.168.1.1"),
            String::from("10.0.0.1"),
            String::from("172.16.0.1"),
            String::from("224.0.0.1"),
            String::from("127.0.0.1"),
            String::from("169.254.1.1"),
            String::from("100.64.0.1"),
            String::from("9.9.9.9"),
            String::from("45.33.32.156"),
            String::from("185.199.109.153"),
            String::from("185.60.216.35"),
            String::from("203.113.0.0"),
            String::from("46.101.0.1"),
            String::from("192.0.78.24"),
            String::from("185.89.218.1"),
            String::from("2.16.0.0"),
            String::from("185.15.56.1"),
            String::from("91.121.0.0"),
            String::from("23.236.62.147"),
            String::from("192.16.48.187"),
            String::from("199.7.83.42"),
            String::from("192.5.5.241"),
            String::from("202.12.27.33"),
            String::from("192.112.36.4"),
            String::from("192.203.230.10"),
            String::from("195.113.144.201"),
            String::from("204.62.14.1"),
            String::from("209.85.231.104"),
            String::from("151.101.65.195"),
            String::from("63.245.215.20"),
            String::from("91.198.174.192"),
            String::from("208.80.154.224"),
            String::from("204.79.197.200"),
            String::from("64.233.160.0"),
            String::from("205.251.192.0"),
            String::from("198.252.206.25"),
            String::from("64.34.119.12"),
            String::from("38.112.0.1"),
            String::from("198.41.30.241"),
            String::from("66.249.64.0"),
            String::from("129.6.13.37"),
            String::from("134.102.0.1"),
            String::from("152.2.210.81"),
            String::from("192.203.230.10"),
            String::from("128.183.112.122"),
            String::from("131.107.0.89"),
            String::from("137.229.64.18"),
            String::from("132.239.1.101"),
            String::from("198.137.240.91"),
            String::from("156.33.241.5"),
            String::from("157.240.20.35"),
            String::from("69.63.176.13"),
            String::from("172.217.14.206"),
            String::from("17.172.224.47"),
            String::from("104.244.42.129"),
            String::from("185.60.216.35"),
            String::from("185.199.108.153"),
            String::from("204.79.197.200"),
            String::from("66.220.144.0"),
            String::from("31.13.71.36"),
            String::from("192.30.255.112"),
            String::from("151.101.129.140"),
            String::from("52.58.78.16"),
            String::from("45.60.0.0"),
            String::from("52.84.0.0"),
            String::from("216.58.216.164"),
            String::from("104.96.0.0"),
            String::from("34.200.0.0"),
            String::from("151.101.1.140"),
            String::from("104.18.43.42"),
            String::from("8.8.8.8"),
            String::from("8.8.4.4"),
            String::from("1.1.1.1"),
            String::from("1.0.0.1"),
            String::from("13.107.21.200"),
            String::from("52.95.110.1"),
            String::from("34.117.59.81"),
            String::from("104.16.132.229"),
            String::from("20.185.79.1"),
            String::from("54.239.28.85"),
            String::from("3.5.140.0"),
            String::from("35.190.247.1"),
            String::from("151.101.1.69"),
            String::from("198.41.0.4"),
            String::from("208.67.222.222"),
            String::from("23.21.224.150"),
            String::from("204.246.162.1"),
            String::from("192.0.2.1"),
            String::from("203.0.113.1"),
            String::from("198.51.100.1"),
        ],
    };

    let start = std::time::Instant::now();
    socket.send(&request.encode_to_vec()).await?;

    let mut data = [0u8; 65535];
    let len = socket.recv(&mut data).await?;

    let response = protos::IpLookupResponse::decode(&data[..len])?;

    let time_took = start.elapsed();

    println!("Received {} bytes in {}us:\n", len, time_took.as_micros());
    for (ip, result) in response.results {
        println!("'{}' : {:#?}", ip, result);
    }

    Ok(())
}
