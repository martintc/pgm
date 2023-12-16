use cli_table::Table;
use postgres_types::FromSql;

use super::replication_state::ReplicationState;

#[derive(Debug, FromSql, Table)]
pub struct Node {
    #[table(name = "formation_id")]
    pub formationid: String, //text
    #[table(skip)]
    pub nodeid: i64, // bigint
    #[table(skip)]
    pub groupid: i32, // integer
    #[table(skip)]
    pub nodename: String, // text
    #[table(name = "node_host")]
    pub nodehost: String, // text
    #[table(skip)]
    pub nodeport: i32, // integer
    #[table(skip)]
    pub systemidentifier: i64, // bigint
    #[table(skip)]
    pub goalstate: ReplicationState, // replication_state
    #[table(name = "reported_state")]
    pub reportedstate: ReplicationState, // replication_state
    #[table(name = "is_running")]
    pub reportedpgisrunning: bool, // bool
    #[table(skip)]
    pub reportedrepstate: String, // text
    #[table(skip)]
    pub reporttime: std::time::SystemTime, // time stamp w/ timezone
    #[table(skip)]
    pub reportedtli: i32, // integer
    #[table(skip)]
    pub reportedlsn: postgres_types::PgLsn, // pg_lsn
    #[table(skip)]
    pub walreporttime: std::time::SystemTime, // timestamp w/ timezone
    #[table(name = "health")]
    pub health: i32, // integer
    #[table(skip)]
    pub healthchecktime: std::time::SystemTime, // timestamp w/timezone
    #[table(skip)]
    pub statechangetime: std::time::SystemTime, // timestampe w/ timezone
    #[table(skip)]
    pub candidatepriority: i32, // integer
    #[table(skip)]
    pub replicationquorum: bool, // boolean
    #[table(skip)]
    pub nodecluster: String, // text
}

impl Node {
    pub fn from_row(row: &postgres::Row) -> Node {
        Node {
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
        }
    }
}
