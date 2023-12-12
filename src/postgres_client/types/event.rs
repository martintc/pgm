use postgres_types::FromSql;

use super::replication_state::ReplicationState;

#[derive(Debug, FromSql)]
pub struct Event {
    eventid: i64, // bigint
    eventtime: std::time::SystemTime, // timestamp w/timezone
    formationid: String, // text
    nodeid: i64, // bigint
    groupip: i32, // integer
    nodename: String, // text
    nodehost: String, // text
    nodeport: i32, // integer
    reportedstate: ReplicationState, // replication_state
    goalstate: ReplicationState, // replication_state
    reportedtli: i32, // integer
    reportedlsn: postgres_types::PgLsn, // pglsn
    candidatepriority: i32, // integer
    replicationquorum: bool, // boolean
    description: String, // text
}