use postgres::Client;
use postgres_types::{ FromSql };
use anyhow::{ Error, Result };

use crate::{models::monitor::Monitor, utility};

#[derive(Debug, FromSql)]
#[postgres(name = "replication_state")]
enum ReplicationState {
    #[postgres(name = "unknown")]
    Unknown,

    #[postgres(name = "init")]
    Init,

    #[postgres(name = "single")]
    Single,

    #[postgres(name = "wait_primary")]
    WaitPrimary,

    #[postgres(name = "draining")]
    Draining,

    #[postgres(name = "demote_timeout")]
    DemoteTimeout,

    #[postgres(name = "demoted")]
    Demoted,

    #[postgres(name = "catchingup")]
    CatchingUp,

    #[postgres(name = "prepare_promotion")]
    PreparePromotion,

    #[postgres(name = "stop_replication")]
    StopReplication,

    #[postgres(name = "wait_standby")]
    WaitStandby,

    #[postgres(name = "maintenance")]
    Maintenance,

    #[postgres(name = "join_primary")]
    JoinPrimary,

    #[postgres(name = "apply_settings")]
    ApplySettings,

    #[postgres(name = "prepare_maintenance")]
    PrepareMaintenace,

    #[postgres(name = "wait_maintenance")]
    WaitMaintenance,

    #[postgres(name = "report_lsn")]
    ReportLsn,

    #[postgres(name = "fast_forward")]
    FastForward,

    #[postgres(name = "join_secondary")]
    JoinSecondary,

    #[postgres(name = "dropped")]
    Dropped,

    #[postgres(name = "primary")]
    Primary,

    #[postgres(name = "secondary")]
    Secondary
}

pub fn show_state(monitor: Monitor) -> Result<(), Error> {
    let connection_string = utility::connection_string::create_connection_string(monitor);
    let mut client = Client::connect(connection_string.as_str(), postgres::NoTls)?;

    let results = client.query("select * from pgautofailover.node", &[])?;

    for row in results.into_iter() {
        // let value: Option<&[u8]> = row.try_get(1)?;
        // println!("{}", value.unwrap());
        // println!("{:#?}", row);
        let i: String = row.try_get(4)?;
        let j: i32 = row.try_get(5)?;
        let k: ReplicationState = row.try_get(7)?;
        let l: ReplicationState = row.try_get(8)?;
        println!("{} {} {:#?} {:#?}", i, j, k, l);
    }

    Ok(())
}