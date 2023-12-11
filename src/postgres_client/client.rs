use postgres::Client;
use anyhow::{ Error, Result };

use crate::{models::monitor::Monitor, utility};

use crate::postgres_client::types::replication_state::ReplicationState;

pub fn show_state(monitor: Monitor) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let results = client.query("select * from pgautofailover.node", &[])?;

    for row in results.into_iter() {
        let i: String = row.try_get(4)?;
        let j: i32 = row.try_get(5)?;
        let k: ReplicationState = row.try_get(7)?;
        let l: ReplicationState = row.try_get(8)?;
        println!("{} {} {:#?} {:#?}", i, j, k, l);
    }

    Ok(())
}