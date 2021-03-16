extern crate clap;

use std::error::Error;
use clap::{Arg, App};

mod duck_dns;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let params: duck_dns::DuckDns = parse_args();

    if params.is_valid() {
        params.send_request().await?;
    }

    Ok(())
}

fn parse_args() -> duck_dns::DuckDns {

    let matches = App::new("Simple command line for Duck Dns API")

        .version("0.1")

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

    let params = duck_dns::DuckDns::new(
        String::from(matches.value_of("token").unwrap()),
        String::from(matches.value_of("domain").unwrap()),
        ip
    );

    params
}
