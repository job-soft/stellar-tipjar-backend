use std::time::Duration;
use tracing;

pub struct QueryLogger;

impl QueryLogger {
    pub fn log_query(query: &str, duration: Duration) {
        let duration_ms = duration.as_millis();
        if duration_ms > 100 {
            tracing::warn!(
                target: "sql_monitoring",
                "SLOW QUERY ({}ms): {}",
                duration_ms,
                query.trim().replace('\n', " ")
            );
        } else {
            tracing::debug!(
                target: "sql_monitoring",
                "Query executed in {}ms: {}",
                duration_ms,
                query.trim().replace('\n', " ")
            );
        }
    }
}
