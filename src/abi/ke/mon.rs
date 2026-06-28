use crate::*;

Ke!
{
    /// Logging function with formatting support.
    ///
    /// This function constructs a formatted message, then iterates over all
    /// registered sinks and writes the message to each one.
    ///
    /// # Parameters
    /// - `level`: log level
    /// - `module`: module path (from `module_path!()`)
    /// - `file`: source file name (from `file!()`)
    /// - `line`: line number (from `line!()`)
    /// - `args`: format arguments (from `format_args!()`)
    KeMonLog                @   fn  (level: KeAttLvl, module: KeStr, file: KeStr, line: u32, args: Arguments<'_>)

    /// Adds a sink to the global registry.
    ///
    /// The sink must be `'static` (i.e., either a `static` variable or a leaked reference).
    KeMonAddSink            @   fn  (sink: &'static mut dyn KeSink)
}
