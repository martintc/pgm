use std::time::SystemTime;

use anyhow::{Error, Result};
use chrono::{DateTime, Local};
use postgres::Client;
use postgres_types::Type;

use crate::{models::monitor::Monitor, utility};

use crate::postgres_client::types::node::Node;

use super::types::event::Event;
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

pub fn show_last_x_events(monitor: Monitor, last_x_events: i64) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let statement = client.prepare_typed("select * from pgautofailover.event order by eventtime desc limit $1", &[Type::INT8])?;

    let results = client.query(&statement, &[&last_x_events])?;

    let mut events: Vec<Event> = vec![];

    for row in results.into_iter() {
        let event = Event {
            eventid: row.get(0),
            eventtime: row.get(1),
            formationid: row.get(2),
            nodeid: row.get(3),
            groupip: row.get(4),
            nodename: row.get(5),
            nodehost: row.get(6),
            nodeport: row.get(7),
            reportedstate: row.get(8),
            goalstate: row.get(9),
            reportedrepstate: row.get(10),
            reportedtli: row.get(11),
            reportedlsn: row.get(12),
            candidatepriority: row.get(13),
            replicationquorum: row.get(14),
            description: row.get(15),
        };

        events.push(event);
    }

    for event in events.iter() {
        println!("{} [{}] - {} - {}", event.eventid, DateTime::<Local>::from(event.eventtime), event.nodehost, event.description);
    } 

    Ok(())
}
