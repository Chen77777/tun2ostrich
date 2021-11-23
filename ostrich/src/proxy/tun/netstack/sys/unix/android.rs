use std::{
    io::{self, ErrorKind, IoSlice},
    marker::Unpin,
};

use tokio::io::{AsyncWrite, AsyncWriteExt};
use tun::platform::Device as TunDevice;

/// Packet Information length in bytes
pub const IFF_PI_PREFIX_LEN: usize = 0;

/// Prepending Packet Information
///
/// ```
/// +--------+--------+--------+--------+
/// | Flags (0)       | Protocol        |
/// +--------+--------+--------+--------+
/// ```
/*pub async fn write_packet_with_pi<W: AsyncWrite + Unpin>(
    writer: &mut W,
    packet: &[u8],
) -> io::Result<()> {
    if packet.is_empty() {
        return Err(io::Error::new(ErrorKind::InvalidInput, "empty packet"));
    }

    let mut header = [0u8; 4];

    // Protocol, infer from the original packet
    let protocol = match packet[0] >> 4 {
        4 => libc::ETH_P_IP,
        6 => libc::ETH_P_IPV6,
        _ => {
            return Err(io::Error::new(
                ErrorKind::InvalidData,
                "neither an IPv4 or IPv6 packet",
            ))
        }
    };

    let protocol_buf = &mut header[2..];
    let protocol_bytes = (protocol as u16).to_be_bytes();
    protocol_buf.copy_from_slice(&protocol_bytes);

    // let bufs = [IoSlice::new(&header), IoSlice::new(packet)];
    // let n = writer.write_vectored(&bufs).await?;
    let m = writer.write(&header).await?;
    let n = writer.write(packet).await?;

    // Packets must be written together with the header
    if (m + n) != (header.len() + packet.len()) {
        return Err(io::Error::new(
            ErrorKind::Other,
            format!(
                "write_vectored header {} bytes, packet {} bytes, but sent {} bytes",
                header.len(),
                packet.len(),
                (m + n)
            ),
        ));
    }

    Ok(())
}*/
pub async fn write_packet_with_pi<W: AsyncWrite + Unpin>(
    writer: &mut W,
    packet: &[u8],
) -> io::Result<()> {
    writer.write_all(packet).await
}
/// Set platform specific route configuration
pub async fn set_route_configuration(_device: &TunDevice) -> io::Result<()> {
    Ok(())
}
