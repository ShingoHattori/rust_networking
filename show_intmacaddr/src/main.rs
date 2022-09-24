extern crate pnet;
 
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket, MutableEthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use std::env;
 

fn main() {
    // インターフェース名を引き数で取得
    let network_interface1 = env::args().nth(1).unwrap();

    let interfaces1 = datalink::interfaces();

    let interface1 = interfaces1.into_iter().filter(|interface: &NetworkInterface| interface.name == network_interface1).next().expect("Failed get Interface1");

    println!("Inteface1:{}",interface1.name);

    let mac = interface1.mac;
    if let Some(mac) = mac {
       println!("mac1:{:?}",mac);
    }
}
