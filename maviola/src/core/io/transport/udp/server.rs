use std::net::{Ipv4Addr, SocketAddr, ToSocketAddrs};

use crate::core::io::{ConnectionConf, ConnectionDetails, ConnectionInfo};
use crate::core::utils::net::resolve_socket_addr;

use crate::prelude::*;

/// TCP server configuration.
///
/// Provides connection configuration for a node that binds to a UDP port and communicates with
/// remote UDP connections.
///
/// Each incoming connection will be considered as a separate channel.
///
/// Use [`UdpClient`] to create a TCP client node.
///
/// # Usage
///
/// Create a synchronous UDP server node:
///
/// ```rust,no_run
/// # #[cfg(feature = "sync")] {
/// use maviola::prelude::*;
///
/// let addr = "127.0.0.1:14500";
///
/// // Create a UDP server node
/// let node = Node::sync::<V2>()
///         /* define other node parameters */
/// #       .system_id(1)
/// #       .component_id(1)
///         .connection(
///             UdpServer::new(addr)    // Configure UDP server connection
///                 .unwrap()
///         ).build().unwrap();
/// # }
/// ```
///
/// Create an asynchronous UDP server node:
///
/// ```rust,no_run
/// # #[cfg(not(feature = "async"))] fn main() {}
/// # #[cfg(feature = "async")]
/// # #[tokio::main] async fn main() {
/// use maviola::prelude::*;
///
/// let addr = "127.0.0.1:14500";
///
/// // Create a UDP server node
/// let node = Node::asnc::<V2>()
///         /* define other node parameters */
/// #       .system_id(1)
/// #       .component_id(1)
///         .connection(
///             UdpServer::new(addr)    // Configure UDP server connection
///                 .unwrap()
///         ).build().await.unwrap();
/// # }
/// ```
#[derive(Clone, Debug)]
pub struct UdpServer {
    pub(crate) addr: SocketAddr,
    pub(crate) info: ConnectionInfo,
    pub(crate) multicast_addr: Option<Ipv4Addr>,
    pub(crate) multicast_interface: Option<Ipv4Addr>,
    pub(crate) ttl: Option<u32>,
}

impl UdpServer {
    /// Instantiates a UDP server configuration.
    ///
    /// Accepts as `addr` anything that implements [`ToSocketAddrs`], prefers IPv4 addresses if
    /// available.
    pub fn new(addr: impl ToSocketAddrs) -> Result<Self> {
        let addr = resolve_socket_addr(addr)?;
        let info = ConnectionInfo::new(ConnectionDetails::UdpServer { bind_addr: addr });
        Ok(Self {
            addr,
            info,
            multicast_addr: None,
            multicast_interface: None,
            ttl: None,
        })
    }

    /// This function specifies a new multicast group for this socket to join.
    /// Executes `UdpSocket::join_multicast_v4` under the hood.
    pub fn with_multicast_v4(self, multiaddr: Ipv4Addr, interface: Ipv4Addr) -> Self {
        Self {
            addr: self.addr,
            info: self.info,
            multicast_addr: Some(multiaddr),
            multicast_interface: Some(interface),
            ttl: None,
        }
    }

    /// Sets the time-to-live field that is used in every packet sent from this socket.
    /// Executes `UdpSocket::set_ttl` under the hood.
    pub fn with_ttl(self, ttl: u32) -> Self {
        Self {
            addr: self.addr,
            info: self.info,
            multicast_addr: self.multicast_addr,
            multicast_interface: self.multicast_interface,
            ttl: Some(ttl),
        }
    }
}

impl ConnectionConf for UdpServer {
    fn info(&self) -> &ConnectionInfo {
        &self.info
    }
}
