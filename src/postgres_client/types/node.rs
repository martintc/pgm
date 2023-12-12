use postgres_types::{ FromSql };

use super::replication_state::ReplicationState;

#[derive(Debug, Default, FromSql)]
pub struct Node {
    pub formationid: Option<String>, //text
    pub nodeid: Option<i64>, // bigint
    pub groupid: Option<i32>, // integer
    pub nodename: Option<String>, // text
    pub nodehost: Option<String>, // text
    pub nodeport: Option<i32>, // integer
    pub systemidentifier: Option<i64>, // bigint
    pub goalstate: Option<ReplicationState>, // replication_state
    pub reportedstate: Option<ReplicationState>, // replication_state
    pub reportedpgisrunning: bool, // bool
    pub reportedrepstate: Option<String>, // text
    pub reporttime: Option<std::time::SystemTime>, // time stamp w/ timezone
    pub reportedtli: Option<i32>, // integer
    pub reportedlsn: Option<postgres_types::PgLsn>, // pg_lsn
    pub walreporttime: Option<std::time::SystemTime>, // timestamp w/ timezone
    pub health: Option<i32>, // integer
    pub healthchecktime: Option<std::time::SystemTime>, // timestamp w/timezone
    pub statechangetime: Option<std::time::SystemTime>, // timestampe w/ timezone
    pub candidatepriority: Option<i32>, // integer
    pub replicationquorum: bool, // boolean
    pub nodecluster: Option<String>, // text
}