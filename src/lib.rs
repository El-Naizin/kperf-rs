use libc::{c_int, size_t};

#[link(name="kperf", kind="framework")]
extern "C" {
    /// Print current CPU identification string to the buffer (same as snprintf),
    /// such as "cpu_7_8_10b282dc_46". This string can be used to locate the PMC
    /// database in /usr/share/kpep.
    /// @return string's length, or negative value if error occurs.
    /// @note This method does not requires root privileges.
    /// @details sysctl get(hw.cputype), get(hw.cpusubtype),
    ///                 get(hw.cpufamily), get(machdep.cpu.model)
    fn kpc_cpu_string(buf: *mut u8, buf_size: size_t) -> c_int;

    fn kpc_pmu_version() -> u32;
}

pub fn cpu_string() -> Option<Vec<u8>> {
    unsafe {
        let buf_size: size_t = 256;
        let mut buf = Vec::with_capacity(buf_size as usize);
        let buf_ptr = buf.as_mut_ptr();

        let res_len = kpc_cpu_string(buf_ptr, buf_size);
        if res_len > 0 {
            buf.set_len(res_len as usize);
            Some(buf)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIBRARY_PATH: &str = "/System/Library/PrivateFrameworks/kperf.framework/kperf";

    #[test]
    fn test_running_function() {
        let pmu_version = unsafe {
            kpc_pmu_version()
        };
    }

    #[test]
    fn test_pmu_version() {
        let pmu_version = unsafe {
            kpc_pmu_version()
        };
        assert_ne!(pmu_version, 0, "A PMU version of zero means an error, try to run this program in sudo");
    }

    #[test]
    fn test_get_cpu_string() {
        let cpu_version = cpu_string();
        // print!("CPU version: ");
        // for c in cpu_version.unwrap() {
        //     print!("{}", c as char);
        // }
        assert_ne!(cpu_version, None, "CPU Version identifier cannot be null");
        assert_ne!(cpu_version.unwrap().len(), 0, "CPU Version identifier string cannot be of size 0");
    }
}
