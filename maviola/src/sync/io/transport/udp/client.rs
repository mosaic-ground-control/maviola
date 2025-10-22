use crate::core::io::ChannelDetails;
use crate::core::utils::SharedCloser;
use crate::sync::io::transport::udp::udp_rw::UdpRW;
use crate::sync::io::{Connection, ConnectionBuilder, ConnectionHandler};
use crate::sync::marker::ConnConf;

use crate::prelude::*;

impl<V: MaybeVersioned> ConnectionBuilder<V> for UdpClient {
    fn build(&self) -> Result<(Connection<V>, ConnectionHandler)> {
        let bind_addr = self.sock.local_addr()?;
        let server_addr = self.sock.peer_addr()?;

        let udp_socket = self.sock.try_clone()?;

        let writer = UdpRW::new(udp_socket);
        let reader = writer.try_clone()?;

        let (connection, chan_factory) = Connection::new(self.info.clone(), SharedCloser::new());

        let chan_info = connection
            .info()
            .make_channel_info(ChannelDetails::UdpClient {
                server_addr,
                bind_addr,
            });
        let channel = chan_factory.build(chan_info, reader, writer);
        let channel_state = channel.spawn();

        let handler = ConnectionHandler::spawn_from_state(channel_state);

        Ok((connection, handler))
    }

    fn to_conf(&self) -> ConnConf<V> {
        ConnConf::new(self.clone())
    }

    fn is_repairable(&self) -> bool {
        true
    }
}
