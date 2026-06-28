macro _ke($($(->)? $x:ident $(,)?)+){$(mod$x;pub use$x::*;)+}

_ke!
{
    ->  mon
    ->  mem
    ->  event
    ->  paging
    ->  module
    ->  device
    ->  exec
    ->  fs
}
