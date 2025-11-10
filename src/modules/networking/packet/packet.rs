pub const HEADER_BYTES: usize = 2;

#[derive(Clone, Copy)]
pub enum PacketType {
    Keyboard,
    Video
}

impl PacketType {
    pub fn from_bytes(byte: u8) -> PacketType {
        match byte {
            000000 => PacketType::Video,
            000001 => PacketType::Keyboard,
            _ => unreachable!()
        }
    }

    pub fn to_bytes(packet_type: PacketType) -> u8 {
        match packet_type {
            PacketType::Video => 000000,
            PacketType::Keyboard => 000001,
        }
    }
}

pub struct Packet {
    packet_type: PacketType,
    data: Vec<u8>,
}

impl Packet {
    pub fn new(packet_type: PacketType, data: Vec<u8>) -> Packet {
        Packet {
            packet_type,
            data,
        }
    }

    /// 1 byte: packet type
    /// 4 bytes: data length
    /// n bytes: data
    pub fn serialize(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = Vec::with_capacity(self.data.len() + HEADER_BYTES);

        // type
        buffer.push(PacketType::to_bytes(self.packet_type));

        // data len
        for byte in self.data.len().to_be_bytes() {
            buffer.push(byte);
        }
        
        // data
        for byte in &self.data {
            buffer.push(byte.clone());
        }

        return buffer;
    }

    pub fn deserialize(buffer: Vec<u8>) -> Packet {
        let packet_type = PacketType::from_bytes(buffer[0]);

        let (_, data_len, _) = unsafe { buffer[1..5].align_to::<u32>() };

        let data = buffer[5..].to_vec();

        Packet {
            packet_type,
            data,
        }
    }
}