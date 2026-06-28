pub enum KeFormat {
    Pretty,
    Regular,
}

pub trait KeSink: core::fmt::Write + Sync + Send {
    fn format(&self) -> KeFormat;
}
