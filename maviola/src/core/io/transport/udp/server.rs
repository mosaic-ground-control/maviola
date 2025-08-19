use crate::core::io::{ConnectionConf, ConnectionDetails, ConnectionInfo};
use std::net::UdpSocket;
use std::sync::Arc;

use crate::prelude::*;

/// TCP server configuration.
///
/// Provides connection configuration for a node that binds to a UDP port and communicates with
/// remote UDP connections.
///
/// Each incoming connection will be considered as a separate channel.
///
/// Use [`UdpClient`] to create a TCP client node.
#[derive(Clone, Debug)]
pub struct UdpServer {
    pub(crate) info: ConnectionInfo,
    pub(crate) sock: Arc<UdpSocket>,
}

impl UdpServer {
    /// Instantiates a UDP server configuration.
    ///
    /// Accepts pre-configured [`UdpSocket`].
    pub fn new(sock: UdpSocket) -> Result<Self> {
        let info = ConnectionInfo::new(ConnectionDetails::UdpServer {
            bind_addr: sock.local_addr()?,
        });
        Ok(Self {
            info,
            sock: Arc::new(sock),
        })
    }
}

impl ConnectionConf for UdpServer {
    fn info(&self) -> &ConnectionInfo {
        &self.info
    }
}
