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
