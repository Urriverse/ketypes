use crate::*;

Ke!
{
    KeModuleLoad            @   fn  (data: &'_ [u8]) -> Result<KeModuleHandle<'_>, String>

    KeModuleSymbols         @   fn  (m: KeModuleHandle<'_>) -> Result<Vec<KeSymbol>, String>

    KeModuleString          @   fn  (m: KeModuleHandle<'_>, st_name: u32) -> Result<String, String>

    KeModulePointer         @   fn  (m: KeModuleHandle<'_>, sym: KeSymbol) -> *const ()

    KeModuleExecute         @   fn  (m: KeModuleHandle<'_>) -> KeTaskId
}
