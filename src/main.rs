use std::error::Error;

use zbus::Connection;

mod avahi;

fn domain_to_rdata(domain: &str) -> Vec<u8> {
    // Convert a domain into mDNS data record format (rdata)

    fn label_to_rdata(label: &str) -> Vec<u8> {
        let mut rlabel = vec![label.len() as u8];
        rlabel.extend(label.as_bytes().to_vec());
        rlabel
    }

    let mut rdata: Vec<u8> = domain
        .split('.')
        .flat_map(|label| label_to_rdata(label))
        .collect();
    rdata.push(0 as u8);
    rdata
}

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
