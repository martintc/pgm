use anyhow::{Error, Result};
use cli_table::{print_stdout, WithTitle};
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
        results = client.query(
            "select * from pgautofailover.node where formationid = $1",
            &[&formation],
        )?;
    } else {
        results = client.query("select * from pgautofailover.node", &[])?;
    }

    let mut nodes: Vec<Node> = vec![];

    for row in results.into_iter() {
        nodes.push(Node::from_row(&row));
    }

    print_stdout(nodes.iter().with_title())?;

    Ok(())
}

pub fn show_primaries_or_secondaries(
    monitor: Monitor,
    desired_state: ReplicationState,
) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let results = client.query("select * from pgautofailover.node", &[])?;

    let mut nodes: Vec<Node> = vec![];

    for row in results.into_iter() {
        let node = Node::from_row(&row);

        if node.reportedstate == desired_state {
            nodes.push(node);
        }
    }

    print_stdout(nodes.iter().with_title())?;

    Ok(())
}

pub fn show_last_x_events(monitor: Monitor, last_x_events: i64) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let statement = client.prepare_typed(
        "select * from pgautofailover.event order by eventtime desc limit $1",
        &[Type::INT8],
    )?;

    let results = client.query(&statement, &[&last_x_events])?;

    let mut events: Vec<Event> = vec![];

    for row in results.into_iter() {
        events.push(Event::from_row(&row));
    }

    print_stdout(events.iter().with_title())?;

    Ok(())
}
