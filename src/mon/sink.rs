pub enum Format {
    Pretty,
    Regular,
}

pub trait Sink: core::fmt::Write + Sync {
    fn format() -> Format;
}
