use super::cbc;
use super::hmacsha1;
use rand::RngCore;

#[derive(Debug)]
pub struct ESP {
    seq: u32,
    spi: u32,
    enc_key: [u8; 16],
    mac_key: [u8; 20],
    iv: [u8; 16],
}

impl ESP {
    pub fn new(seq: u32, spi: u32, enc_key: [u8; 16], mac_key: [u8; 20]) -> Self {
        let mut rng = rand::thread_rng();
        let mut iv = [0u8; 16];
        rng.fill_bytes(&mut iv);
        ESP {
            seq,
            spi,
            enc_key,
            mac_key,
            iv,
        }
    }
}

#[derive(Debug)]
pub struct ESPPacket {
    spi: u32,
    seq: u32,
    iv: [u8; 16],
    data: Vec<u8>,
}

impl ESPPacket {
    pub fn new(esp: &ESP, data: Vec<u8>) -> Self {
        ESPPacket {
            spi: esp.spi,
            seq: esp.seq,
            iv: esp.iv,
            data,
        }
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        let spi = u32::from_be_bytes(data[0..4].try_into().unwrap());
        let seq = u32::from_be_bytes(data[4..8].try_into().unwrap());
        let iv = data[8..24].try_into().unwrap();
        let data = data[24..].to_vec();

        Ok(ESPPacket { spi, seq, iv, data })
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut packet = Vec::new();
        packet.extend_from_slice(&self.spi.to_be_bytes());
        packet.extend_from_slice(&self.seq.to_be_bytes());
        packet.extend_from_slice(&self.iv);
        packet.extend_from_slice(&self.data);
        packet
    }

    pub fn encrypt(&mut self, esp: &ESP) -> Result<(), Box<dyn std::error::Error>> {
        let blocksize = 16;

        // 填充数据
        let padding_len = blocksize - 1 - (self.data.len() % blocksize);
        for i in 1..padding_len {
            self.data.push(i as u8);
        }
        self.data.push((padding_len - 1) as u8);

        let nexthdr: u8 = 0x04;
        self.data.push(nexthdr);

        // 加密数据 (AES-128-CBC)
        let data = cbc::encrypt(&esp.enc_key, &self.data, &esp.iv);
        self.data = data;

        // 计算 HMAC
        let hmac = hmacsha1::hmac_sha1(&esp.mac_key, &self.data);
        self.data.extend_from_slice(&hmac);

        Ok(())
    }

    pub fn decrypt(&mut self, esp: &ESP) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // 验证 HMAC
        let hmac = &self.data[self.data.len() - 20..];
        let data = &self.data[..self.data.len() - 20];
        let expected_hmac = hmacsha1::hmac_sha1(&esp.mac_key, data);
        if expected_hmac != hmac {
            return Err("HMAC verification failed".into());
        }

        // 解密数据 (AES-128-CBC)
        let data = cbc::decrypt(&esp.enc_key, data, &self.iv);

        // 去除填充数据
        let padding_len = data[data.len() - 1] as usize;
        let data = &data[..data.len() - padding_len];

        Ok(data.to_vec())
    }
}
