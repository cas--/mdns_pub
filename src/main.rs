use std::error::Error;

use zbus::Connection;

mod avahi;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection = Connection::system().await?;

    let avahi_server = avahi::server::ServerProxy::new(&connection).await?;

    let reply = avahi_server.get_host_name_fqdn().await?;
    dbg!(reply);

    let reply2 = avahi_server
        .resolve_host_name(-1, -1, "test.local", -1, 0)
        .await?;
    dbg!(reply2);
    Ok(())
}
