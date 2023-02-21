use std::error::Error;
use std::time::Duration;

use async_std::task;
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
    println!("Server: {}", avahi_server.get_version_string().await?);

    let hostname = avahi_server.get_host_name_fqdn().await?;
    println!("Hostname: {hostname}");

    let cname = "test.local";

    let entry_group_path = avahi_server.entry_group_new().await?;
    dbg!("group: {}", entry_group_path.as_str());

    let entry_group = avahi::entry_group::EntryGroupProxy::builder(&connection)
        .path(&entry_group_path)?
        .build()
        .await?;

    let rdata: Vec<u8> = domain_to_rdata(cname);

    entry_group
        .add_record(-1, -1, 0, cname, 1, 5, 1, rdata)
        .await?;
    entry_group.commit().await?;

    loop {
        println!("Resolving hostname {cname}...");
        task::sleep(Duration::from_secs(3)).await;
        let resolved_hostname = avahi_server.resolve_host_name(-1, -1, cname, -1, 0).await;
        match resolved_hostname {
            Ok(result) => println!("{result:?}"),
            Err(error) => println!("Problem with resolve: {error}"),
        };
    }
    Ok(())
}
