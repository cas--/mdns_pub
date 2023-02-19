use std::error::Error;

use zbus::{dbus_proxy, Connection};

#[dbus_proxy(
    interface = "org.freedesktop.Avahi.Server",
    default_service = "org.freedesktop.Avahi",
    default_path = "/"
)]
trait AvahiServer {
    // Call Notify D-Bus method
    fn GetHostNameFqdn(&self) -> zbus::Result<String>;
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let avahi_server = AvahiServerProxy::new(&connection).await?;

    let reply = avahi_server.GetHostNameFqdn().await?;
    dbg!(reply);
    Ok(())
}
