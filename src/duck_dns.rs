extern crate uuid;
extern crate hyper;

use std::net::IpAddr;
use std::error::Error;

use uuid::Uuid;
use hyper::Client;
use hyper_tls::HttpsConnector;

pub struct DuckDns {
    pub token: String,
    pub domain: String,
    pub ip: String
}

impl DuckDns {

    pub fn new(token: String, domain: String, ip: String) -> DuckDns {
        DuckDns {
            token: token,
            domain: domain,
            ip: ip,
        }
    }

    pub fn is_valid(&self) -> bool {

        // Validate token uuid
        let _uuid: Uuid = match &self.token.parse() {
            Ok(uuid) => *uuid,
            Err(err) => {
                println!("Error - Invalid uuid: {}", err);
                return false;
            }
        };

        // Validate ip address
        if !&self.ip.is_empty() {

            let ip: IpAddr = match &self.ip.parse() {
                Ok(ip_addr) => *ip_addr,
                Err(err) => {
                    println!("Error - Invalid ip: {}", err);
                    return false;
                }
            };

            if !ip.is_ipv6() && !ip.is_ipv4() {
                println!("Not valid ip address");
                return false;
            } else {
                println!("Valid ip address");
            }
        }


        true
    }

    pub async fn send_request(&self) -> Result<bool, Box<dyn Error>> {

        // Still inside `async fn main`...
        let https = HttpsConnector::new();
        let client = Client::builder().build::<_, hyper::Body>(https);

        // Build url
        let url = format!("{}{}{}{}{}{}",
            String::from("https://www.duckdns.org/update?domains="),
            self.domain,
            String::from("&token="),
            self.token,
            String::from("&ip="),
            self.ip
        ).parse()?;

        println!("Sending request: {}", &url);

        // Await the response...
        client.get(url).await?;
        Ok(true)
    }
}
