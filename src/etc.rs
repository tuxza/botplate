use sysinfo::System;

pub struct SysInfo {
    kernel: Option<String>,
    os_vers: Option<String>,
    used_memory: u64,
    total_memory: u64,
}

pub async fn get_sysinfo() -> SysInfo {
    let mut sys = System::new();
    sys.refresh_all();
    SysInfo {
        kernel: System::kernel_version(),
        os_vers: System::os_version(),
        used_memory: sys.used_memory(),
        total_memory: sys.total_memory(),
    }
}
