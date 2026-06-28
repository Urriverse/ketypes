use crate::*;

Ke!
{
    KeExecPanic             @   fn  (info: &PanicInfo) -> !

    KeExecExit              @   fn  (code: KeExitCode) -> !

    KeExecYield             @   fn  ()

    KeExecSleep             @   fn  (KeWaitQueue)

    KeExecSpawn             @   fn  (entry: fn(), pri: KePriority, name: String, affinity: Option<KeCpuId>, new_pid: bool) -> KeTaskId

    KeExecArgumentedSpawn   @   fn  (entry: fn(KeAbstract<8>), arg: KeAbstract<8>, pri: KePriority, name: String, affinity: Option<KeCpuId>, new_pid: bool) -> KeTaskId

    KeExecWaitChild         @   fn  (child_id: KeTaskId) -> KeExitCode

    KeExecSetDeadline       @   fn  (task_id: KeTaskId, deadline_ms: u64)
}
