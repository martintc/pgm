use chrono::{DateTime, Utc};
use cli_table::Table;
use postgres_types::FromSql;

use super::replication_state::ReplicationState;

#[derive(Debug, FromSql, Table)]
pub struct Event {
    #[table(name = "event_id")]
    pub eventid: i64, // bigint
    #[table(name = "event_time")]
    pub eventtime: DateTime<Utc>,
    #[table(skip)]
    pub formationid: String, // text
    #[table(skip)]
    pub nodeid: i64, // bigint
    #[table(skip)]
    pub groupip: i32, // integer
    #[table(skip)]
    pub nodename: String, // text
    #[table(name = "node_host")]
    pub nodehost: String, // text
    #[table(skip)]
    pub nodeport: i32, // integer
    #[table(skip)]
    pub reportedstate: ReplicationState, // replication_state
    #[table(skip)]
    pub goalstate: ReplicationState, // replication_state
    #[table(skip)]
    pub reportedrepstate: String, // text
    #[table(skip)]
    pub reportedtli: i32, // integer
    #[table(skip)]
    pub reportedlsn: postgres_types::PgLsn, // pglsn
    #[table(skip)]
    pub candidatepriority: i32, // integer
    #[table(skip)]
    pub replicationquorum: bool, // boolean
    #[table(name = "description")]
    pub description: String, // text
}

impl Event {
    pub fn from_row(row: &postgres::Row) -> Event {
        Event {
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
        }
    }
}
