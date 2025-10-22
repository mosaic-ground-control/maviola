use async_trait::async_trait;
use tokio::net::UdpSocket;

use crate::asnc::io::transport::udp::udp_rw::UdpRW;
use crate::asnc::io::{Connection, ConnectionBuilder, ConnectionHandler};
use crate::asnc::marker::AsyncConnConf;
use crate::core::io::ChannelDetails;
use crate::core::utils::SharedCloser;

use crate::prelude::*;

#[async_trait]
impl<V: MaybeVersioned> ConnectionBuilder<V> for UdpClient {
    async fn build(&self) -> Result<(Connection<V>, ConnectionHandler)> {
        let bind_addr = self.sock.local_addr()?;
        let server_addr = self.sock.peer_addr()?;

        let udp_socket = self.sock.try_clone()?;
        let udp_socket = UdpSocket::from_std(udp_socket)?;

        let writer = UdpRW::new(udp_socket);
        let reader = writer.clone();

        let (connection, chan_factory) = Connection::new(self.info.clone(), SharedCloser::new());

        let chan_info = connection
            .info()
            .make_channel_info(ChannelDetails::UdpClient {
                server_addr,
                bind_addr,
            });
        let channel = chan_factory.build(chan_info, reader, writer);
        let channel_state = channel.spawn().await;

        let handler = ConnectionHandler::spawn_from_state(channel_state);

        Ok((connection, handler))
    }

    fn to_conf(&self) -> AsyncConnConf<V> {
        AsyncConnConf::new(self.clone())
    }

    fn is_repairable(&self) -> bool {
        true
    }
}
