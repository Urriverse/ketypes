use crate::{KeAbstract, KeNutex};

pub type KeWaitQueue = KeNutex<KeAbstract<8>>;

pub type KeTaskId = u64;

pub type KePriority = i32;

pub type KeExitCode = i32;
