use pnet::packet::icmp::echo_request::MutableEchoRequestPacket;
use pnet::packet::icmp::{self, IcmpPacket, IcmpTypes};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::{checksum, MutableIpv4Packet};
use pnet::packet::{udp, Packet};
use std::net::Ipv4Addr;

use super::esp::{self, ESPPacket, ESP};
use super::udp::UdpClient;

const MAGIC_PING_PAYLOAD: &[u8; 16] = b"monitor\x00\x00pan ha ";

pub fn send_probes(esp: &ESP) -> Result<ESPPacket, Box<dyn std::error::Error>> {
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
    ipv4_packet.set_source(Ipv4Addr::new(10, 193, 129, 116));
    ipv4_packet.set_destination(Ipv4Addr::new(10, 193, 33, 1));

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
    let icmp_checksum = icmp::checksum(&IcmpPacket::new(icmp_packet.packet()).unwrap());
    icmp_packet.set_checksum(icmp_checksum);

    // 将 ICMP 数据包附加到 IPv4 数据包
    ipv4_packet.set_payload(icmp_packet.packet());

    // 计算并设置 IPv4 校验和
    let ipv4_checksum = checksum(&ipv4_packet.to_immutable());
    ipv4_packet.set_checksum(ipv4_checksum);

    // 打印构建的 IP 数据包
    let data = ipv4_packet.packet();
    let mut esppacket = ESPPacket::new(&esp, data.try_into().unwrap());
    esppacket.encrypt(&esp)?;
    Ok(esppacket)
}

pub fn catch_probes(pkt: &ESPPacket) -> Result<bool, Box<dyn std::error::Error>> {
    let mut data = pkt.data.clone();

    // 校验icmp数据包内容是否为MAGIC_PING_PAYLOAD
    let icmp_packet = MutableEchoRequestPacket::new(&mut data[20..]).unwrap();
    let icmp_payload = icmp_packet.payload();
    println!("Received packet: {:?}", hex::encode(&icmp_payload));
    if icmp_payload.len() != MAGIC_PING_PAYLOAD.len() || icmp_payload != MAGIC_PING_PAYLOAD {
        return Ok(false);
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;

    #[tokio::test]
    async fn test_send_probes() {
        let esp_out = ESP::new(
            1u32,
            0x54C277B0,
            hex::decode("ce30001139e8cde103b214d7af0509e4")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("f53b81dae1db0d5754ef52edc1516be18ca749f5")
                .unwrap()
                .try_into()
                .unwrap(),
        );

        let esp_in = ESP::new(
            1u32,
            0x28E7990F,
            hex::decode("a76d929d37613210f60bd233f2806b32")
                .unwrap()
                .try_into()
                .unwrap(),
            hex::decode("0734369faa973a05f44dd0bb19d4559f10ed41b8")
                .unwrap()
                .try_into()
                .unwrap(),
        );
        // 创建一个UDP socket
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

        // 目标地址
        let addr = "47.100.9.56:4501";
        // 尝试连接到目标地址
        socket.connect(addr).unwrap();

        // 输出连接成功
        println!("连接成功到: {}", addr);

        let pkt2send = send_probes(&esp_out).unwrap();
        let data2send = pkt2send.to_bytes();

        // 发送数据到目标地址
        socket.send(&data2send).unwrap();
        println!("已发送数据: {:?}", hex::encode(&data2send));

        // 接收数据缓冲区
        let mut recv_buffer = [0; 2048];

        // 接收数据
        let (amt, src) = socket.recv_from(&mut recv_buffer).unwrap();
        println!("从 {} 接收到 {} 字节数据", src, amt);

        // 输出接收到的数据
        println!("接收到的数据: {:?}", &recv_buffer[..amt]);

        let received_data = &recv_buffer[..amt];

        let mut pkt2catch = ESPPacket::from_bytes(received_data).unwrap();
        pkt2catch.decrypt(&esp_in).unwrap();
        println!("解密后的数据: {:?}", hex::encode(pkt2catch.to_bytes()));
        let is_valid = catch_probes(&pkt2catch).unwrap();

        assert_eq!(is_valid, true);
    }
}
