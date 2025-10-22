use crate::core::io::{ConnectionConf, ConnectionDetails, ConnectionInfo};
use std::net::UdpSocket;
use std::sync::Arc;

use crate::prelude::*;

/// UDP client configuration.
///
/// Provides connection configuration for a node that communicates with a specified UDP port. Use
/// [`UdpServer`] to create a UDP server node.
#[derive(Clone, Debug)]
pub struct UdpClient {
    pub(crate) info: ConnectionInfo,
    pub(crate) sock: Arc<UdpSocket>,
}

impl UdpClient {
    /// Instantiates a UDP client configuration.
    ///
    /// Accepts pre-configured [`UdpSocket`].
    pub fn new(sock: UdpSocket) -> Result<Self> {
        let info = ConnectionInfo::new(ConnectionDetails::UdpClient {
            remote_addr: sock.peer_addr()?,
        });
        Ok(Self {
            info,
            sock: Arc::new(sock),
        })
    }
}

impl ConnectionConf for UdpClient {
    fn info(&self) -> &ConnectionInfo {
        &self.info
    }
}
