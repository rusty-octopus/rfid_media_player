pub(crate) trait Logging {
    fn info(description: &str);
    fn error(description: &str);
    fn debug(description: &str);
}
