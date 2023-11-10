pub mod structs;
pub mod functions;
pub mod constants;



// pub fn cpu_string() -> Option<Vec<u8>> {
//     unsafe {
//         let buf_size: size_t = 256;
//         let mut buf = Vec::with_capacity(buf_size as usize);
//         let buf_ptr = buf.as_mut_ptr();
//
//         let res_len = kpc_cpu_string(buf_ptr, buf_size);
//         if res_len > 0 {
//             buf.set_len(res_len as usize);
//             Some(buf)
//         } else {
//             None
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use crate::{structs::*, constants::*, functions::* };
    use std::ptr::{null, null_mut};
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

    // #[test]
    // fn test_get_cpu_string() {
    //     let cpu_version = cpu_string();
    //     // print!("CPU version: ");
    //     // for c in cpu_version.unwrap() {
    //     //     print!("{}", c as char);
    //     // }
    //     assert_ne!(cpu_version, None, "CPU Version identifier cannot be null");
    //     assert_ne!(cpu_version.unwrap().len(), 0, "CPU Version identifier string cannot be of size 0");
    // }

    #[test]
    fn test_create_kpep_db() {
        unsafe {
            let mut db_ptr: *mut kpep_db = null_mut();
            // let mut name: &[c_char; 12] = b"test_rust_db";
            // let name = CStr::from_bytes_until_nul().unwrap();
            // let name_ptr: *const c_char = name.as_ptr() as *const c_char;
            let result = kpep_db_create(null(), &mut db_ptr);
            println!("db creation result: {}", result);
        }
    }
}
