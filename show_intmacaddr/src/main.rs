extern crate pnet;
 
use pnet::datalink::{self, NetworkInterface};

use std::env;
 

fn main() {
    // インターフェース名を引き数で取得．
    let network_interface1 = env::args().nth(1).unwrap();

    let interfaces1 = datalink::interfaces();

    let interface1 = interfaces1.into_iter().filter(|interface: &NetworkInterface| interface.name == network_interface1).next().expect("Failed get Interface1");

    println!("Inteface1:{}",interface1.name);

    let mac = interface1.mac;

    // interfaceがnullの場合はmac addrも当然nullになりうるので，macはswiftのoptionalみたいになる．なので，unwrapみたいなことをする．
    if let Some(mac) = mac {
      // mac addr構造体をいい感じにstringに変換する．
      println!("mac1:{:?}",mac);
    }
}
