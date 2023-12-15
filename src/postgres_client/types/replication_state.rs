use core::fmt;

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

impl fmt::Display for ReplicationState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ReplicationState::Unknown => write!(f, "unknown"),
            ReplicationState::Init => write!(f, "init"),
            ReplicationState::Single => write!(f, "single"),
            ReplicationState::WaitPrimary => write!(f, "wait_primary"),
            ReplicationState::Draining => write!(f, "draining"),
            ReplicationState::DemoteTimeout => write!(f, "demote_timeout"),
            ReplicationState::Demoted => write!(f, "demoted"),
            ReplicationState::CatchingUp => write!(f, "catching_up"),
            ReplicationState::PreparePromotion => write!(f, "prepare_promotion"),
            ReplicationState::StopReplication => write!(f, "stop_replication"),
            ReplicationState::WaitStandby => write!(f, "wait_stand_by"),
            ReplicationState::Maintenance => write!(f, "maintenance"),
            ReplicationState::JoinPrimary => write!(f, "join_primary"),
            ReplicationState::ApplySettings => write!(f, "apply_settings"),
            ReplicationState::PrepareMaintenace => write!(f, "prepare_maintenance"),
            ReplicationState::WaitMaintenance => write!(f, "wait_maintenance"),
            ReplicationState::ReportLsn => write!(f, "report_lsn"),
            ReplicationState::FastForward => write!(f, "fast_forward"),
            ReplicationState::JoinSecondary => write!(f, "join_secondary"),
            ReplicationState::Dropped => write!(f, "dropped"),
            ReplicationState::Primary => write!(f, "primary"),
            ReplicationState::Secondary => write!(f, "secondary"),
        }
    }
}
