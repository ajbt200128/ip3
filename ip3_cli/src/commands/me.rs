use local_ip_address::local_ip;
use std::{println, net::IpAddr};
use colored::Colorize;

pub async fn display_me(json:bool) {
    let public_ip_ipv4 = public_ip::addr().await.unwrap();
    let local_ip_ipv4 = local_ip().unwrap();

    match (public_ip_ipv4, local_ip_ipv4){
        (IpAddr::V4(public_ip), IpAddr::V4(local_ip)) => {
            let public_ip_ip3 = ip3::ipv4_to_ip3(public_ip);
            let local_ip_ip3 = ip3::ipv4_to_ip3(local_ip);

            if json {
                println!("{{\"public_ip\": {{\"ipv4\":\"{}\", \"ip3\":\"{}\"}}, \"local_ip\": {{\"ipv4\":\"{}\", \"ip3\":\"{}\"}}}}", public_ip, public_ip_ip3, local_ip, local_ip_ip3);
            }else {
                println!("{} {}","Public IP: ".bold().bright_blue(), String::from(public_ip_ip3).bright_green());
                println!("{} {}","Local IP: ".bold().bright_blue(), String::from(local_ip_ip3).bright_green());
            }
        }
        _ => {
            println!("No IPv4 address found!");
        }
    }
}
