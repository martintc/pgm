use postgres_types::{ FromSql };

use super::replication_state::ReplicationState;

#[derive(Debug, FromSql)]
pub struct Node {
    pub formationid: String, //text
    pub nodeid: i64, // bigint
    pub groupid: i32, // integer
    pub nodename: String, // text
    pub nodehost: String, // text
    pub nodeport: i32, // integer
    pub systemidentifier: i64, // bigint
    pub goalstate: ReplicationState, // replication_state
    pub reportedstate: ReplicationState, // replication_state
    pub reportedpgisrunning: bool, // bool
    pub reportedrepstate: String, // text
    pub reporttime: std::time::SystemTime, // time stamp w/ timezone
    pub reportedtli: i32, // integer
    pub reportedlsn: postgres_types::PgLsn, // pg_lsn
    pub walreporttime: std::time::SystemTime, // timestamp w/ timezone
    pub health: i32, // integer
    pub healthchecktime: std::time::SystemTime, // timestamp w/timezone
    pub statechangetime: std::time::SystemTime, // timestampe w/ timezone
    pub candidatepriority: i32, // integer
    pub replicationquorum: bool, // boolean
    pub nodecluster: String, // text
}