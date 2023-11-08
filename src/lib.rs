/// Exactly what I need:
/// https://users.rust-lang.org/t/best-way-to-access-dll-functions-in-rust/14548/8
// use libloading::{Library, Symbol};

// pub struct PerformanceCounters {
//     cycles: f64,
//     branches: f64,
//     missed_branches: f64,
//     instructions: f64,
// }
//
#[link(name="kperf", kind="framework")]
extern "C" {
    fn kpc_pmu_version() -> u32;
}

#[cfg(test)]
mod tests {
    use super::*;
    const LIBRARY_PATH: &str = "/System/Library/PrivateFrameworks/kperf.framework/kperf";

    // #[test]
    // fn loading_library() {
    //     unsafe {
    //         let lib = Library::new(LIBRARY_PATH).unwrap();
    //     }
    // }
    //
    // #[test]
    // fn loading_function() {
    //     unsafe {
    //         let lib = Library::new(LIBRARY_PATH).unwrap();
    //         let func: Symbol<kpc_pmu_version> = lib.get(b"kpc_pmu_version").unwrap();
    //         let res = func();
    //         assert_ne!(res, 0, "PMU Version is 0, which is an error, do you have sudo permissions?");
    //         println!("PMU Version: {}", res);
    //     }
    // }

    #[test]
    fn test_running_function() {
        let a = unsafe {
            kpc_pmu_version()
        };
        println!("STATIC, PMU version: {}", a);
    }
}
