// use gpconnect::libs::esp::{ESPPacket, ESP};
// use gpconnect::libs::gpst::{catch_probes, send_probes};
// use std::net::UdpSocket;
// fn main() {
//     let esp_out = ESP::new(
//         1u32,
//         0x54C277B0,
//         hex::decode("ce30001139e8cde103b214d7af0509e4")
//             .unwrap()
//             .try_into()
//             .unwrap(),
//         hex::decode("f53b81dae1db0d5754ef52edc1516be18ca749f5")
//             .unwrap()
//             .try_into()
//             .unwrap(),
//     );

//     let esp_in = ESP::new(
//         1u32,
//         0x28E7990F,
//         hex::decode("a76d929d37613210f60bd233f2806b32")
//             .unwrap()
//             .try_into()
//             .unwrap(),
//         hex::decode("0734369faa973a05f44dd0bb19d4559f10ed41b8")
//             .unwrap()
//             .try_into()
//             .unwrap(),
//     );
//     // 创建一个UDP socket
//     let socket = UdpSocket::bind("0.0.0.0:0").unwrap();

//     // 目标地址
//     let addr = "47.100.9.56:4501";
//     // 尝试连接到目标地址
//     socket.connect(addr).unwrap();

//     // 输出连接成功
//     println!("连接成功到: {}", addr);

//     let pkt2send = send_probes(&esp_out).unwrap();
//     let data2send = pkt2send.to_bytes();

//     // 发送数据到目标地址
//     socket.send(&data2send).unwrap();
//     println!("已发送数据: {:?}", hex::encode(&data2send));

//     // 接收数据缓冲区
//     let mut recv_buffer = [0; 2048];

//     // 接收数据
//     let (amt, src) = socket.recv_from(&mut recv_buffer).unwrap();
//     println!("从 {} 接收到 {} 字节数据", src, amt);

//     // 输出接收到的数据
//     println!("接收到的数据: {:?}", &recv_buffer[..amt]);

//     let received_data = &recv_buffer[..amt];

//     let mut pkt2catch = ESPPacket::from_bytes(received_data).unwrap();
//     pkt2catch.decrypt(&esp_in).unwrap();
//     println!("解密后的数据: {:?}", hex::encode(pkt2catch.to_bytes()));
//     let is_valid = catch_probes(&pkt2catch).unwrap();
//     println!("数据是否有效: {}", is_valid);
// }

use serde::{Deserialize, Serialize};
use serde_xml_rs::from_str;

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    #[serde(rename = "gw-address")]
    gw_address: String,
    #[serde(rename = "ip-address")]
    ip_address: String,
    ipsec: Ipsec,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ipsec {
    #[serde(rename = "c2s-spi")]
    c2s_spi: String,
    #[serde(rename = "s2c-spi")]
    s2c_spi: String,
    #[serde(rename = "akey-s2c")]
    akey_s2c: Key,
    #[serde(rename = "ekey-s2c")]
    ekey_s2c: Key,
    #[serde(rename = "akey-c2s")]
    akey_c2s: Key,
    #[serde(rename = "ekey-c2s")]
    ekey_c2s: Key,
}

#[derive(Debug, Serialize, Deserialize)]
struct Key {
    bits: String,
    val: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let xml = r#"
    <response status="success">
		<need-tunnel>yes</need-tunnel>
		<ssl-tunnel-url>/ssl-tunnel-connect.sslvpn</ssl-tunnel-url>
		<portal>GP-GW-SHAP-N</portal>
		<user>ling.pcheng@fujitsu.com</user>
		<quarantine>no</quarantine>
		<lifetime>2592000</lifetime>
		<timeout>10800</timeout>
		<disconnect-on-idle>10800</disconnect-on-idle>
		<bw-c2s>1000</bw-c2s>
		<bw-s2c>1000</bw-s2c>
		<gw-address>10.193.33.1</gw-address>
		<ipv6-connection>no</ipv6-connection>
		<ip-address>10.193.129.116</ip-address>
		<netmask>255.255.255.255</netmask>
		<ip-address-preferred>yes</ip-address-preferred>
		<ip-address-v6>fc00::1abb</ip-address-v6>
		<ip-address-v6-preferred>yes</ip-address-v6-preferred>
		<dns-v6>
			<member>10.12.255.254</member>
		</dns-v6> 
		<dns>
			<member>10.12.255.254</member>
		</dns> 
		<wins>
		</wins> 
		<dns-suffix>
		</dns-suffix> 
		<default-gateway>10.193.129.116</default-gateway>
		<default-gateway-v6>fc00::1abb</default-gateway-v6>
		<mtu>0</mtu>
		<no-direct-access-to-local-network>no</no-direct-access-to-local-network>
		<access-routes>
			<member>0.0.0.0/0</member>
			<member>10.12.255.254/32</member>
		</access-routes> 
		<access-routes-v6>
			<member>::/0</member>
		</access-routes-v6> 
		<exclude-access-routes>
		</exclude-access-routes> 
		<exclude-access-routes-v6>
		</exclude-access-routes-v6> 
		<ipsec>
			<udp-port>4501</udp-port>
			<ipsec-mode>esp-tunnel</ipsec-mode>
			<enc-algo>aes-128-cbc</enc-algo>
			<hmac-algo>sha1</hmac-algo>
			<c2s-spi>0x54C277B0</c2s-spi>
			<s2c-spi>0x28E7990F</s2c-spi>
			<akey-s2c>
				<bits>160</bits>
				<val>0734369faa973a05f44dd0bb19d4559f10ed41b8</val>
			</akey-s2c> 
			<ekey-s2c>
				<bits>128</bits>
				<val>a76d929d37613210f60bd233f2806b32</val>
			</ekey-s2c> 
			<akey-c2s>
				<bits>160</bits>
				<val>f53b81dae1db0d5754ef52edc1516be18ca749f5</val>
			</akey-c2s> 
			<ekey-c2s>
				<bits>128</bits>
				<val>ce30001139e8cde103b214d7af0509e4</val>
			</ekey-c2s> 
		</ipsec> 
	</response>
    "#;

    let response: Response = from_str(xml)?;

    println!("gw-address: {}", response.gw_address);
    println!("ip-address: {}", response.ip_address);
    println!("c2s-spi: {}", response.ipsec.c2s_spi);
    println!("s2c-spi: {}", response.ipsec.s2c_spi);
    println!(
        "akey-s2c: {} bits, {}",
        response.ipsec.akey_s2c.bits, response.ipsec.akey_s2c.val
    );
    println!(
        "ekey-s2c: {} bits, {}",
        response.ipsec.ekey_s2c.bits, response.ipsec.ekey_s2c.val
    );
    println!(
        "akey-c2s: {} bits, {}",
        response.ipsec.akey_c2s.bits, response.ipsec.akey_c2s.val
    );
    println!(
        "ekey-c2s: {} bits, {}",
        response.ipsec.ekey_c2s.bits, response.ipsec.ekey_c2s.val
    );

    Ok(())
}
