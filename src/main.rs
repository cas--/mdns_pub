use std::error::Error;

use zbus::{dbus_proxy, Connection};

mod avahi;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let avahi_server = avahi::server::ServerProxy::new(&connection).await?;

    let reply = avahi_server.get_host_name_fqdn().await?;
    dbg!(reply);
    Ok(())
}
