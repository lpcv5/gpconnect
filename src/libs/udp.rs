use std::error::Error;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::mpsc;

pub struct UdpClient {
    socket: Arc<UdpSocket>,
    server_addr: SocketAddr,
    tx: mpsc::Sender<Vec<u8>>,
}

impl UdpClient {
    pub async fn new(
        local_addr: &str,
        server_addr: &str,
    ) -> Result<Self, Box<dyn Error + Send + Sync>> {
        let socket = UdpSocket::bind(local_addr).await?;
        let server_addr: SocketAddr = server_addr.parse()?;
        let (tx, mut rx) = mpsc::channel::<Vec<u8>>(100);

        let socket = Arc::new(socket);
        let socket_clone = Arc::clone(&socket);

        println!("UdpClient listening on {}", local_addr);

        // 启动发送任务
        tokio::spawn(async move {
            while let Some(data) = rx.recv().await {
                if let Err(e) = socket_clone.send_to(&data, &server_addr).await {
                    eprintln!("Failed to send data: {:?}", e);
                }
            }
        });

        Ok(UdpClient {
            socket,
            server_addr,
            tx,
        })
    }

    pub async fn send(&self, data: Vec<u8>) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.tx.send(data).await?;
        Ok(())
    }

    pub async fn receive(&self) -> Result<(Vec<u8>, SocketAddr), Box<dyn Error + Send + Sync>> {
        let mut buf = vec![0u8; 1024];
        let (len, addr) = self.socket.recv_from(&mut buf).await?;
        buf.truncate(len);
        Ok((buf, addr))
    }

    pub async fn run_receive_loop<F>(
        &self,
        mut callback: F,
    ) -> Result<(), Box<dyn Error + Send + Sync>>
    where
        F: FnMut(Vec<u8>, SocketAddr) + Send + 'static,
    {
        loop {
            let (data, addr) = self.receive().await?;
            callback(data, addr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use tokio::net::UdpSocket;
    use tokio::time::{sleep, Duration};

    async fn run_mock_server(addr: SocketAddr) -> Result<(), Box<dyn Error + Send + Sync>> {
        let socket = UdpSocket::bind(addr).await?;
        println!("Mock server listening on {}", addr);

        let mut buf = vec![0u8; 1024];
        let (len, client_addr) = socket.recv_from(&mut buf).await?;
        println!("Server received: {}", String::from_utf8_lossy(&buf[..len]));

        let response = b"Hello, client!";
        socket.send_to(response, client_addr).await?;
        println!("Server sent response");

        Ok(())
    }

    #[tokio::test]
    async fn test_udp_client() -> Result<(), Box<dyn Error + Send + Sync>> {
        // 启动模拟服务器
        let server_addr = "127.0.0.1:8090".parse::<SocketAddr>()?;
        let server_handle = tokio::spawn(async move { run_mock_server(server_addr).await });

        // 等待服务器启动
        sleep(Duration::from_millis(100)).await;

        // 创建客户端
        let client = UdpClient::new("127.0.0.1:0", "127.0.0.1:8090").await?;

        // 发送数据
        client.send(b"Hello, server!".to_vec()).await?;
        println!("Client sent message");

        // 接收数据
        let (received_data, _) = client.receive().await?;
        let received_message = String::from_utf8_lossy(&received_data);
        println!("Client received: {}", received_message);

        // 验证接收到的消息
        assert_eq!(received_message, "Hello, client!");

        // 等待服务器完成
        let _ = server_handle.await??;

        Ok(())
    }
}
