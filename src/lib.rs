#[link(name="kperf", kind="framework")]
extern "C" {
    fn kpc_pmu_version() -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIBRARY_PATH: &str = "/System/Library/PrivateFrameworks/kperf.framework/kperf";

    #[test]
    fn test_running_function() {
        let a = unsafe {
            kpc_pmu_version()
        };
        println!("STATIC, PMU version: {}", a);
    }
}
