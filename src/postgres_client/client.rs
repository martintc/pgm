use postgres::Client;
use anyhow::{ Error, Result };
use postgres_types::FromSql;

use crate::{models::monitor::Monitor, utility};

use crate::postgres_client::types::replication_state::ReplicationState;
use crate::postgres_client::types::node::Node;

pub fn show_state(monitor: Monitor) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let results = client.query("select * from pgautofailover.node", &[])?;

    let mut nodes: Vec<Node> = vec![];

    for row in results.into_iter() {
        // let i: String = row.try_get(4)?;
        // let j: i32 = row.try_get(5)?;
        // let k: ReplicationState = row.try_get(7)?;
        // let l: ReplicationState = row.try_get(8)?;
        // println!("{} {} {:#?} {:#?}", i, j, k, l);
        // let mut node = Node { ..Default::default() };
        // let values = Node::from_sql(row, )?;
        let node = Node {
            formationid: row.get(0),
            nodeid: row.get(1),
            groupid: row.get(2),
            nodename: row.get(3),
            nodehost: row.get(4),
            nodeport: row.get(5),
            systemidentifier: row.get(6),
            goalstate: row.get(7),
            reportedstate: row.get(8),
            reportedpgisrunning: row.get(9),
            reportedrepstate: row.get(10),
            reporttime: row.get(11),
            reportedtli: row.get(12),
            reportedlsn: row.get(13),
            walreporttime: row.get(14),
            health: row.get(15),
            healthchecktime: row.get(16),
            statechangetime: row.get(17),
            candidatepriority: row.get(18),
            replicationquorum: row.get(19),
            nodecluster: row.get(20),
        };

        println!("{}", node.nodename.unwrap());
    }



    Ok(())
}