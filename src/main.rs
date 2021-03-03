extern crate hyper;
extern crate clap;
extern crate uuid;

use hyper::Client;
use hyper_tls::HttpsConnector;
use clap::{Arg, App};
use uuid::Uuid;

use std::net::IpAddr;

struct DuckDnsApiParams {
    token: String,
    domain: String,
    ip: String
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    let params: DuckDnsApiParams = parse_args();

    println!("{}", params.token);
    println!("{}", params.domain);
    println!("{}", params.ip);

    // Validate args
    let _uuid = match Uuid::parse_str(params.token.as_str()) {
        Ok(uuid) => uuid,
        Err(err) => {
            println!("Error - Invalid uuid: {}", err);
            return Ok(())
        }
    };

    if !params.ip.is_empty() {

        let ip: IpAddr = match params.ip.parse() {

            Ok(ip_addr) => ip_addr,
            Err(err) => {
                println!("Error - Invalid ip: {}", err);
                return Ok(())
            }
        };

        if !ip.is_ipv6() && !ip.is_ipv4() {
            println!("Not valid ip address");
        } else {
            println!("Valid ip address");
        }
    }

    // Still inside `async fn main`...
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    // Build url
    let url = format!("{}{}{}{}{}{}",
        String::from("https://www.duckdns.org/update?domains="),
        params.domain,
        String::from("&token="),
        params.token,
        String::from("&ip="),
        params.ip
    ).parse()?;

    println!("Sending request: {}", &url);

    // Await the response...
    let resp = client.get(url).await?;

    println!("Response: {}", resp.status());

    Ok(())
}

fn parse_args() -> DuckDnsApiParams {

    let matches = App::new("My Super Program")

        .version("0.1")
        .author("k")
        .about("Does awesome things")

        .arg(Arg::with_name("domain")
            .short("d")
            .long("domain")
            .value_name("DOMAIN")
            .help("Domain name")
            .takes_value(true)
            .required(true))

        .arg(Arg::with_name("token")
            .short("t")
            .long("token")
            .value_name("TOKEN")
            .help("Token uuid")
            .takes_value(true)
            .required(true))

        .arg(Arg::with_name("ip")
            .short("i")
            .long("ip")
            .value_name("IP")
            .help("ip address")
            .takes_value(true)
            .required(false))

        .get_matches();

    let ip: String = if matches.is_present("ip") {
        String::from(matches.value_of("ip").unwrap())
    } else {
        String::from("")
    };

    let params = DuckDnsApiParams {
        token: String::from(matches.value_of("token").unwrap()),
        domain: String::from(matches.value_of("domain").unwrap()),
        ip: ip
    };

    params
}
