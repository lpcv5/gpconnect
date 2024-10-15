use hex;

use gpconnect::libs::esp::{ESP, ESPPacket};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_hex =
        "4500002cdc4c40004001a5480ac121010ac182b90000103c474702cd6d6f6e69746f72000070616e20686120";
    let data = hex::decode(data_hex)?;

    let mut esp_out = ESP::new(
        1u32,
        0x6f77893a,
        hex::decode("510f909f4014dfec78b3bb8c7cbe86ac")?
            .try_into()
            .unwrap(),
        hex::decode("678c7e80dd68ee69e1279da28054186de9ec113c")?
            .try_into()
            .unwrap(),
    );
    let mut esppacket = ESPPacket::new(&mut esp_out, data.clone());



    Ok(())
}
