use postgres_types::FromSql;

#[derive(Debug, Default, FromSql, PartialEq)]
#[postgres(name = "replication_state")]
pub enum ReplicationState {
    #[default]
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
    Secondary,
}
