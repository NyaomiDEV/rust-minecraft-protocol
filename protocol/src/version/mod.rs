pub mod forge_v1_13;
pub mod v1_14_4;
pub mod v1_16_5;
pub mod v1_17;
pub mod v1_17_1;

/// Trait to obtain packet ID from packet data.
pub trait PacketId {
    /// Get protcol packet ID.
    fn packet_id(&self) -> u8;
}

#[macro_export]
macro_rules! set_packet_id (
    ($type: ident, $id: expr) => (
        impl $type {
            const PACKET_ID: u8 = $id;
        }

        impl PacketId for $type {
            fn packet_id(&self) -> u8 {
                Self::PACKET_ID
            }
        }
    )
);
