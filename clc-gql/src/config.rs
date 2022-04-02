/// Environment variable configurations.
pub mod env {

    /// Logging filter directive.
    pub static LOG_DIRECTIVES: &str = "CLCGQL_LOG";

    /// Control logging color.
    pub static LOG_COLOR: &str = "CLCGQL_LOG_COLOR";

    /// Redis connection info.
    pub static REDIS_ENDPOINT: &str = "CLCGQL_REDIS_ENDPOINT";

    /// Return true if log is to be displayed in color.
    pub fn enable_log_color() -> bool {
        std::env::var(LOG_COLOR)
            .ok()
            .and_then(|raw| raw.parse::<bool>().ok())
            .unwrap_or(false)
    }
}
