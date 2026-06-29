pub type KeDevice = Arc![Device];

pub type KeMethodId = u64;

pub type KeDeviceId = u32;

pub type KeDeviceMethod = extern "C" fn(KeDeviceId, usize) -> KeDeviceResult;

extrum::extrum! {
    #[derive(Clone, Copy, PartialEq)]
    pub enum KeDeviceStatus: usize {
        SUCCESS = 0,
        NOT_FOUND = 1,
        INVALID_ARG = 2,
        BUSY = 3,
        IO_ERROR = 4,
        UNSUPPORTED = usize::MAX,
    }
}

#[repr(C)]
pub struct KeDeviceResult {
    pub value: usize,
    pub status: KeDeviceStatus,
}
