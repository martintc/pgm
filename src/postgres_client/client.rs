use anyhow::{Error, Result};
use postgres::Client;

use crate::{models::monitor::Monitor, utility};

use crate::postgres_client::types::node::Node;

use super::types::replication_state::ReplicationState;

pub fn show_state(monitor: Monitor, formation: Option<String>) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let mut results: Vec<postgres::Row> = vec![];

    if let Some(formation) = formation {
        results = client.query("select * from pgautofailover.node where formationid = $1", &[&formation])?;
    } else {
        results = client.query("select * from pgautofailover.node", &[])?;
    }

    // let mut nodes: Vec<Node> = vec![];

    for row in results.into_iter() {
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

        println!("{}", node.nodename);
    }

    Ok(())
}

pub fn show_primaries_or_secondaries(monitor: Monitor, desired_state: ReplicationState) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let results = client.query("select * from pgautofailover.node", &[])?;

    let mut nodes: Vec<Node> = vec![];

    for row in results.into_iter() {
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

        nodes.push(node);
    }

    for primary in nodes.iter().filter(|node| node.reportedstate == desired_state) {
        println!("{} ({}): {}", primary.nodename, primary.formationid, primary.nodehost);
    }

    Ok(())
}
