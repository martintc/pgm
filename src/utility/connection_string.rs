use crate::models::monitor::Monitor;

pub fn create_connection_string(monitor: Monitor) -> String {
    return format!(
        "host={} port={} user={} password={} dbname={}",
        monitor.host.unwrap(),
        monitor.port.unwrap(),
        monitor.user.unwrap(),
        monitor.password.unwrap(),
        monitor.database_name.unwrap()
    );
}
