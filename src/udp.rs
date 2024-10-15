use std::{
    net::{Ipv4Addr, SocketAddrV4},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

use tokio::{
    io::{split, AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const CMD_UDP: u8 = 1;

pub async fn worker(server: String, udp_worker: tun2layer4::UdpWorker) -> tun2layer4::UdpWorker {
    // UDP
    let mut udp_ctrl_conn = match TcpStream::connect(server.clone()).await {
        Ok(_conn) => _conn,
        Err(_) => {
            return udp_worker;
        }
    };

    if let Err(_) = udp_ctrl_conn.write_u8(CMD_UDP).await {
        return udp_worker;
    }
    log::info!("Connection established.");
    let (mut r, mut w) = split(udp_ctrl_conn);
    let udp_worker2 = udp_worker.clone();
    let running = Arc::new(AtomicBool::new(true));
    let running_clone = running.clone();

    tokio::spawn(async move {
        let mut key = [0u8; 12];
        let mut buf = vec![0u8; 2048];
        while running_clone.load(Ordering::Relaxed) {
            if let Err(_) = r.read_exact(&mut key).await {
                running_clone.store(false, Ordering::Relaxed);
                break;
            }
            let src = SocketAddrV4::new(
                Ipv4Addr::new(key[0], key[1], key[2], key[3]),
                u16::from_be_bytes([key[4], key[5]]),
            );
            let dst = SocketAddrV4::new(
                Ipv4Addr::new(key[6], key[7], key[8], key[9]),
                u16::from_be_bytes([key[10], key[11]]),
            );
            let _len = match r.read_u16().await {
                Ok(n) => n as usize,
                Err(_) => {
                    running_clone.store(false, Ordering::Relaxed);
                    break;
                }
            };
            if let Err(_) = r.read_exact(&mut buf[.._len]).await {
                running_clone.store(false, Ordering::Relaxed);
                break;
            }
            if let Err(_) = udp_worker2.send_back(&buf[.._len], src, dst) {
                running_clone.store(false, Ordering::Relaxed);
                break;
            }
        }
    });

    let mut key = [0u8; 12];
    let mut buf = vec![0u8; 2048];
    while running.load(Ordering::Relaxed) {
        if let Ok((src, dst, size)) = udp_worker.recv_from(&mut buf) {
            key[..4].copy_from_slice(&src.ip().octets());
            key[4..6].copy_from_slice(&src.port().to_be_bytes());
            key[6..10].copy_from_slice(&dst.ip().octets());
            key[10..12].copy_from_slice(&dst.port().to_be_bytes());
            if let Err(_) = w.write_all(&key).await {
                return udp_worker;
            }
            if let Err(_) = w.write_u16(size as u16).await {
                return udp_worker;
            }
            if let Err(_) = w.write_all(&buf[..size]).await {
                return udp_worker;
            }
            if let Err(_) = w.flush().await {
                return udp_worker;
            }
        } else {
            return udp_worker;
        }
    }
    udp_worker
}
