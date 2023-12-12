use postgres_types::FromSql;

use super::replication_state::ReplicationState;

#[derive(Debug, FromSql)]
pub struct Event {
    pub eventid: i64,                       // bigint
    pub eventtime: std::time::SystemTime,   // timestamp w/timezone
    pub formationid: String,                // text
    pub nodeid: i64,                        // bigint
    pub groupip: i32,                       // integer
    pub nodename: String,                   // text
    pub nodehost: String,                   // text
    pub nodeport: i32,                      // integer
    pub reportedstate: ReplicationState,    // replication_state
    pub goalstate: ReplicationState,        // replication_state
    pub reportedrepstate: String,           // text
    pub reportedtli: i32,                   // integer
    pub reportedlsn: postgres_types::PgLsn, // pglsn
    pub candidatepriority: i32,             // integer
    pub replicationquorum: bool,            // boolean
    pub description: String,                // text
}
