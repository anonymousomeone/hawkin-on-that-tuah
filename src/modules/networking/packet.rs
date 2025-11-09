pub enum PacketType {
    Keyboard,
    Video
}

pub struct Packet {
    packet_type: PacketType,
}

impl Packet {
    pub fn new(packet_type: PacketType) -> Packet {
        Packet {
            packet_type,
        }
    }

    pub fn serialize(&self) {

    }
}