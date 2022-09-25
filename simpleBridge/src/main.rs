extern crate pnet;
 
use pnet::datalink::{self, NetworkInterface};
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::MutablePacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;

use std::env;


fn main() {
    // インターフェース名を引き数で取得
    let network_interface1 = env::args().nth(1).unwrap();
    let network_interface2 = env::args().nth(2).unwrap();

    // すべてのネットワークインターフェースを取得
    let interfaces1 = datalink::interfaces();
    let interfaces2 = datalink::interfaces();


    let interface1 = interfaces1.into_iter().filter(|interface: &NetworkInterface| interface.name == network_interface1).next().expect("Failed to get Interface1");

    let interface2 = interfaces2.into_iter().filter(|interface: &NetworkInterface| interface.name == network_interface2).next().expect("Failed to get Interface2");

    println!("Inteface1:{}",interface1.name);
    println!("Inteface2:{}",interface2.name);

    // 送受信に使うソケットを取る
    let (mut tx1, mut rx1) = match datalink::channel(&interface1, Default::default()) {
        // ethernetのソケットとして使えるなら，そいつらを使う．okだけど無が入っていたら，拾えたけど違うやつなのでnot ethernet，エラーならpanic．
        Ok(Ethernet(tx1, rx1)) => (tx1, rx1),
        Ok(_) => panic!("not ethernet"),
        Err(e) => panic!("error ocuured {}", e)
    };


    let (mut tx2, mut rx2) =  match datalink::channel(&interface2, Default::default()) {
        // ethernetのソケットとして使えるなら，そいつらを使う．okだけど無が入っていたら，拾えたけど違うやつなのでnot ethernet，エラーならpanic．
        Ok(Ethernet(tx2, rx2)) => (tx2, rx2),
        Ok(_) => panic!("not ethernet"),
        Err(e) => panic!("error ocuured {}", e)
    };

    let iface1mac = interface1.mac;
    let iface2mac = interface2.mac;


    loop {
        match rx1.next() {
            Ok(packet) => {
            // ここんところで，もう一個のインターフェースから吐き出せばいいんじゃねえのか？
            // macのdstをそれぞれ書き直してあげる必要があるんじゃね?マネージメントができなくなるけど，
            // 受け取ったパケットを，データ部だけそのままにして放出するんか？cのサンプルはそうなってる．バッファをethernet header分だけずらして，こいつを送り出してる．
            
            // 新しくethernetとしてデータの並びをパケットにする
            //tx1.send_to(&packet, None);
            let packet = EthernetPacket::new(packet).unwrap();
            
            tx2.build_and_send(1, packet.packet().len(),
                    &mut |mut new_packet| {
                        let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();

                        // Create a clone of the original packet
                        new_packet.clone_from(&packet);

                        // Switch the source and destination
                        if let Some(iface2mac) = iface2mac {
                            new_packet.set_source(iface2mac);

                        }
                });
            }

            Err(e) => {
            panic!("error occured {}", e);
            }
        }

        
        match rx2.next() {
            Ok(packet) => {
            //let packet = EthernetPacket::new(&packet).unwrap();
            let packet = EthernetPacket::new(packet).unwrap();
            
            tx1.build_and_send(1, packet.packet().len(),
                    &mut |mut new_packet| {
                        let mut new_packet = MutableEthernetPacket::new(new_packet).unwrap();

                        // Create a clone of the original packet
                        new_packet.clone_from(&packet);

                        // Switch the source and destination
                        if let Some(iface1mac) = iface1mac {
                            new_packet.set_source(iface1mac);

                        }
                });
            }

            Err(e) => {
            panic!("error occured {}", e);
            }
        }
    }


















}