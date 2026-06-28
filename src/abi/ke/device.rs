use crate::*;

Ke!
{
    KeVtDeviceNew           @   fn  (KeStr) -> Device

    KeDeviceAddMethod       @   fn  (&mut KeDevice, method_id: KeMethodId, method: KeDeviceMethod)

    KeDeviceGetMethod       @   fn  (&KeDevice, method_id: KeMethodId) -> Option<KeDeviceMethod>

    KeDeviceRegister        @   fn  (device: Box<KeDevice>) -> Option<KeDeviceId>

    KeDeviceUnregister      @   fn  (id: KeDeviceId) -> KeDone

    KeDeviceDataGet         @   fn  (id: KeDeviceId) -> Option<KeAbstract<8>>

    KeDeviceDataSet         @   fn  (id: KeDeviceId, data: KeAbstract<8>) -> KeDone

    KeDeviceMethodInvoke    @   fn  (id: KeDeviceId, method_id: KeMethodId, arg: KeAbstract<8>) -> KeDeviceResult
}
