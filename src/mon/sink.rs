pub enum Format {
    Pretty,
    Regular,
}

pub trait Sink: core::fmt::Write + Sync + Send {
    fn format(&self) -> Format;
}
