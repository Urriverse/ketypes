use crate::*;

Ke!
{
    KeEventSubscribe        @   fn  (event_id: KeEventId, callback: KeEventCallback) -> Result<(), ()>

    KeEventUnsubscribe      @   fn  (event_id: KeEventId, callback: KeEventCallback) -> Result<(), ()>

    KeEventPublish          @   fn  (event_id: KeEventId, data: KeAbstract<8>, affinity: Option<KeCpuId>) -> Result<(), ()>
}
