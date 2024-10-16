use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{self, IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{MutableIpv4Packet, checksum};
use pnet::packet::Packet;
use std::net::Ipv4Addr;

use super::esp::{ESPPacket, ESP};

const MAGIC_PING_PAYLOAD: &[u8; 16] = b"monitor\x00\x00pan ha ";

pub fn send_probes(esp: ESP) -> Result<(), Box<dyn std::error::Error>> {
    // 计算 ICMP 数据包总长度 (ICMP 头部 + 自定义负载)
    let icmp_packet_size = 8 + MAGIC_PING_PAYLOAD.len();

    // 创建一个缓冲区来存储 IP 数据包
    let mut ip_buffer = vec![0u8; 20 + icmp_packet_size]; // 20 字节的 IPv4 头部 + ICMP 数据包
    let mut ipv4_packet = MutableIpv4Packet::new(&mut ip_buffer).unwrap();

    // 设置 IPv4 头部字段
    ipv4_packet.set_version(4);
    ipv4_packet.set_header_length(5);
    ipv4_packet.set_total_length((20 + icmp_packet_size) as u16); // IP 头部 + ICMP 数据包
    ipv4_packet.set_ttl(64);
    ipv4_packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    ipv4_packet.set_source(Ipv4Addr::new(192, 168, 1, 1));
    ipv4_packet.set_destination(Ipv4Addr::new(192, 168, 1, 2));

    // 创建一个缓冲区来存储 ICMP 数据包
    let mut icmp_buffer = vec![0u8; icmp_packet_size]; // ICMP 头部 + 自定义负载
    let mut icmp_packet = MutableEchoRequestPacket::new(&mut icmp_buffer).unwrap();

    // 设置 ICMP 头部字段
    icmp_packet.set_icmp_type(IcmpTypes::EchoRequest);
    icmp_packet.set_identifier(0);
    icmp_packet.set_sequence_number(0);

    // 设置自定义负载数据
    icmp_packet.set_payload(MAGIC_PING_PAYLOAD);

    // 计算 ICMP 校验和
    let icmp_checksum =
        icmp::checksum(&IcmpPacket::new(icmp_packet.packet()).unwrap());
    icmp_packet.set_checksum(icmp_checksum);

    // 将 ICMP 数据包附加到 IPv4 数据包
    ipv4_packet.set_payload(icmp_packet.packet());

    // 计算并设置 IPv4 校验和
    let ipv4_checksum = checksum(&ipv4_packet.to_immutable());
    ipv4_packet.set_checksum(ipv4_checksum);


    // 打印构建的 IP 数据包
    let data = ipv4_packet.packet();
    let mut esppacket = ESPPacket::new(&esp, data.try_into().unwrap());
    match esppacket.encrypt(&esp) {
        Ok(_) => {
            println!("Encrypted packet: {:?}", esppacket.data);
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
    Ok(())
}

mod tests {
    use super::*;

    #[test]
    fn test_send_probes() {
        let esp = ESP::new(
            1u32,
            0x6f77893a,
            hex::decode("510f909f4014dfec78b3bb8c7cbe86ac")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("678c7e80dd68ee69e1279da28054186de9ec113c")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        assert!(send_probes(esp).is_ok());
    }
}
