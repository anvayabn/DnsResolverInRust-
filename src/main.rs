/* This is simple DNS client program written in rust
   What does it do?
   It takes an url as an input and gives the ip address of the
   url/website using googles DNS

   Author: Anvaya B Narappa
   email-id: an001@ucr.edu
*/

use std::net::{Ipv4Addr, IpAddr, SocketAddr};
use std::net::UdpSocket;
use rand::prelude::*;

fn main() -> std::io::Result<()>{

    let ip = Ipv4Addr::new(0, 0, 0, 0);
    let addr = SocketAddr::new(IpAddr::V4(ip), 3400);
    //bind socket to local host
    let socket = UdpSocket::bind(addr).expect("Binding failed");

    let dns_ip = Ipv4Addr::new(8, 8, 8, 8);
    let dns_addr = SocketAddr::new(IpAddr::V4(dns_ip), 53);

    //connect to google dns 8.8.8.8
    socket.connect(dns_addr).unwrap();

    //build DNS query and get back the query to send and the randomly generated transaction id
    let hostname = String::from("github.com");
    let (query, tid) = build_query(hostname.clone());

    print!("Querying {dns_addr} ...");
    socket.send(&query).expect("send failed");

    let mut buf = [0u8; 512];
    let (re ,sa) = socket.recv_from(&mut buf)?;

    // println!("Response : {:?}", &buf[..re]);

    // get the transaction id of the response
    let rtid =  u16::from_be_bytes([buf[0], buf[1]]);
    if tid == rtid {
        // println!("tid is confirmed, they are the same");
         //extract the ip address from the response
        /*
            DNS header length = 12
            hostname - github.com 10 bytes
            count = 2 bytes (represents the length of the hostname);
            type  = 2 bytes
            class = 2 bytes

            need tyo skip these to access the answer
        */
        let ip_offset = 12 + (hostname.len() + 2 ) + 4 ;

        //if type A record
        if buf[ip_offset + 2] == 0x00 && buf[ip_offset + 3] == 0x01 {
            let ip_addr = std::net::Ipv4Addr::new(buf[ip_offset + 12],
                                                  buf[ip_offset + 13],
                                                  buf[ip_offset + 14],
                                                  buf[ip_offset + 15]);
            println!("IP Address of {hostname}: {}", ip_addr);
        }
    }
    Ok(())



}

fn build_query(s: String) -> (Vec<u8>,u16){

    let mut query:Vec<u8> = Vec::new();

    //have a random number for transaction Id
    //generate a random number using rnd
    let mut rng = rand::thread_rng();
    let y = rng.gen::<u16>();

    let transaction_id = y.to_be_bytes();

    query.extend_from_slice(&transaction_id);
    query.extend_from_slice(&[0x01, 0x00]);
    query.extend_from_slice(&[0x00, 0x01]);
    query.extend_from_slice(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

    let hostname = s.clone();

    for part in hostname.split(".") {
        query.push(part.len() as u8);
        query.extend_from_slice(part.as_bytes());
    }

    query.push(0);

    query.extend_from_slice(&[0x00, 0x01]);
    query.extend_from_slice(&[0x00, 0x01]);

    (query, y)

}