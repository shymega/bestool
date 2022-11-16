mod bootloader;
mod errors;
mod memory_info;
mod message;
mod packet;
mod sync;

pub const BES_PROGRAMMING_BAUDRATE: u32 = 921600;
pub const BES_SYNC: u8 = 0xBE;
pub use bootloader::load_programmer_runtime_binary_blob;
pub use bootloader::start_programmer_runtime_binary_blob;
pub use errors::BESLinkError;
pub use memory_info::query_memory_info;
pub use message::BesMessage;
pub use message::MessageTypes;
pub use packet::read_packet;
pub use packet::send_packet;
pub use sync::sync;
