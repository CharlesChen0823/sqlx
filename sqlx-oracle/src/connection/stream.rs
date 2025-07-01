use crate::net::{BufferedSocket, Socket};
use crate::protocol::{Capabilities, Packet, PACKET_HEADER_SIZE};
use std::ops::{Deref, DerefMut};

use sqlx_core::bytes::{Buf, Bytes};
use sqlx_core::io::ProtocolEncode;

use crate::error::Error;
use crate::OracleConnectOptions;

pub struct OracleStream<S = Box<dyn Socket>> {
    // Wrapping the socket in `Box` allows us to unsize in-place.
    pub(crate) socket: BufferedSocket<S>,
    pub(crate) server_version: (u16, u16, u16),
    pub(super) capabilities: Capabilities,
    pub(crate) sequence_id: u8,
    pub(crate) is_tls: bool,
    pub(crate) is_authed: bool,
}

impl<S: Socket> OracleStream<S> {
    pub(crate) fn with_socket(options: &OracleConnectOptions, socket: S) -> Self {
        let capabilities = Capabilities::default();
        Self {
            capabilities,
            server_version: (0, 0, 0),
            sequence_id: 0,
            socket: BufferedSocket::new(socket),
            is_tls: false,
            is_authed: false,
        }
    }

    pub fn boxed_socket(self) -> OracleStream {
        OracleStream {
            socket: self.socket.boxed(),
            server_version: self.server_version,
            capabilities: self.capabilities,
            sequence_id: self.sequence_id,
            is_tls: self.is_tls,
            is_authed: self.is_authed,
        }
    }

    pub(crate) async fn send_packet<'en, T>(&mut self, payload: T) -> Result<(), Error>
    where
        T: ProtocolEncode<'en, Capabilities>,
    {
        self.write_packet(payload)?;
        self.flush().await?;
        Ok(())
    }

    pub(crate) fn write_packet<'en, T>(&mut self, payload: T) -> Result<(), Error>
    where
        T: ProtocolEncode<'en, Capabilities>,
    {
        self.socket.write_with(payload, self.capabilities)
    }

    async fn recv_packet(&mut self) -> Result<Packet<Bytes>, Error> {
        // https://dev.mysql.com/doc/dev/mysql-server/8.0.12/page_protocol_basic_packets.html
        // https://mariadb.com/kb/en/library/0-packet/#standard-packet

        let mut header: Bytes = self.socket.read(PACKET_HEADER_SIZE).await?;

        let packet_size = if self.is_authed {
            header.get_u32()
        } else {
            let packet_size = header.get_u16();
            let _ = header.get_u16();
            packet_size as u32
        };
        let packet_type = header.get_u8();
        let packet_flags = header.get_u8();
        let reserved1 = header.get_u8();
        let reserved2 = header.get_u8();

        let body: Bytes = self
            .socket
            .read(packet_size as usize - PACKET_HEADER_SIZE)
            .await?;

        Ok(Packet {
            packet_size,
            packet_type,
            packet_flags,
            reserved1,
            reserved2,
            body,
        })
    }
}

impl<S> Deref for OracleStream<S> {
    type Target = BufferedSocket<S>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.socket
    }
}

impl<S> DerefMut for OracleStream<S> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.socket
    }
}
