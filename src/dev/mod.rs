pub type Device = crate::Hdl![Device];

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

#[repr(C, align(8))]
pub struct KeDevice {
    pub id: KeDeviceId,
    pub name: alloc::string::String,
    pub parent: Option<KeDeviceId>,
    driver_data: usize,
    methods: alloc::collections::btree_map::BTreeMap<KeMethodId, KeDeviceMethod>,
}
