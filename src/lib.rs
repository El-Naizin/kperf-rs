use libc::{c_int, size_t, c_uint, c_ulonglong, c_char, c_uchar, c_void};


/// KPEP event (size: 48/28 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
struct kpep_event {
    name: *const c_char, ///< Unique name of a event, such as "INST_RETIRED.ANY".
    description: *const c_char, ///< Description for this event.
    errata: *const c_char,      ///< Errata, currently NULL.
    alias: *const c_char,       ///< Alias name, such as "Instructions", "Cycles".
    fallback: *const c_char,    ///< Fallback event name for fixed counter.
    mask: c_uint,
    number: c_uchar,
    umask: c_uchar,
    reserved: c_uchar,
    is_fixed: c_uchar,
}

/// KPEP database (size: 144/80 bytes on 64/32 bit OS)
#[allow(non_camel_case_types)]
#[repr(C)]
struct kpep_db {
    name: *const c_char,           ///< Database name, such as "haswell".
    cpu_id: *const c_char,         ///< Plist name, such as "cpu_7_8_10b282dc".
    marketing_name: *const c_char, ///< Marketing name, such as "Intel Haswell".
    plist_data: *mut c_void,           ///< Plist data (CFDataRef), currently NULL.
    event_map: *mut c_void, ///< All events (CFDict<CFSTR(event_name), kpep_event *>).
    event_arr: *mut kpep_event, ///< Event struct buffer (sizeof(kpep_event) * events_count).
    fixed_event_arr: **mut kpep_event, ///< Fixed counter events (sizeof(kpep_event *)
    ///< * fixed_counter_count)
    alias_map: *mut c_void, ///< All aliases (CFDict<CFSTR(event_name), kpep_event *>).
    reserved_1: size_t,
    reserved_2: size_t,
    reserved_3: size_t,
    event_count: size_t, ///< All events count.
    alias_count: size_t,
    fixed_counter_count: size_t,
    config_counter_count: size_t,
    power_counter_count: size_t,
    archtecture: c_uint, ///< see `KPEP CPU archtecture constants` above.
    fixed_counter_bits: c_uint,
    config_counter_bits: c_uint,
    power_counter_bits: c_uint,
}

/// KPEP config (size: 80/44 bytes on 64/32 bit OS)
#[repr(C)]
#[allow(non_camel_case_types)]
struct kpep_config {
    db: *mut kpep_db,
    ev_arr: **mut kpep_event, ///< (sizeof(kpep_event *) * counter_count), init NULL
    ev_map: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init 0
    ev_idx: *mut size_t,       ///< (sizeof(size_t *) * counter_count), init -1
    flags: *mut c_uint,          ///< (sizeof(c_uint *) * counter_count), init 0
    kpc_periods: *mut c_ulonglong,    ///< (sizeof(c_ulonglong *) * counter_count), init 0
    event_count: size_t,   /// kpep_config_events_count()
    counter_count: size_t,
    classes: c_uint, ///< See `class mask constants` above.
    config_counter: c_uint,
    power_counter: c_uint,
    reserved: c_uint,
}

#[allow(non_camel_case_types)]
type kpc_config_t = u64;

#[link(name="kperf", kind="framework")]
extern "C" {
    fn kpc_cpu_string(buf: *mut u8, buf_size: size_t) -> c_int;
    fn kpc_pmu_version() -> c_uint;
    fn kpc_get_counting() -> c_uint;
    fn kpc_set_counting(classes: c_uint) -> c_int;
    fn kpc_get_thread_counting() -> c_uint;
    fn kpc_set_thread_counting(classes: c_uint) -> c_int;
    fn kpc_get_config_count(classes: c_uint) -> c_uint;
    fn kpc_get_config(classes: c_uint, config: *mut kpc_config_t) -> c_int;
    fn kpc_set_config(classes: c_uint, config: *mut kpc_config_t) -> c_int;
    fn kpc_get_counter_count(classes: c_uint) -> c_uint;
    fn kpc_get_cpu_counters(all_cpus: bool, classes: c_uint, curcpu: *mut c_int, buf: *mut c_ulonglong) -> c_int;
    fn kpc_get_thread_counters(tid: c_uint, buf_count: c_uint, buf: *mut c_ulonglong) -> c_int;
    fn kpc_force_all_ctrs_set(val: c_int) -> c_int;
    fn kpc_force_all_ctrs_get(val_out: *mut c_int) -> c_int;
    fn kperf_action_count_set(count: c_uint) -> c_int;
    fn kperf_action_count_get(count: *mut c_uint) -> c_int;
    fn kperf_action_samplers_set(actionid: c_uint, sample: c_uint) -> c_int;
    fn kperf_action_samplers_get(actionid: c_uint, sample: *mut c_uint) -> c_int;
    fn kperf_action_filter_set_by_task(actionid: c_uint, port: c_int) -> c_int;
    fn kperf_action_filter_set_by_pid(actionid: c_uint, pid: c_int) -> c_int;
    fn kperf_timer_count_set(count: c_uint) -> c_int;
    fn kperf_timer_count_get(count: *mut c_uint) -> c_int;
    fn kperf_timer_period_set(actionid: c_uint, tick: c_ulonglong) -> c_int;
    fn kperf_timer_period_get(actionid: c_uint, tick: *mut c_ulonglong) -> c_int;
    fn kperf_timer_action_set(actionid: c_uint, timerid: c_uint) -> c_int;
    fn kperf_timer_action_get(actionid: c_uint, timerid: *mut c_uint) -> c_int;
    fn kperf_timer_pet_set(timerid: c_uint) -> c_int;
    fn kperf_timer_pet_get(timerid: *mut c_uint) -> c_int;
    fn kperf_sample_set(enabled: c_uint) -> c_int;
    fn kperf_sample_get(enabled: *mut c_uint) -> c_int;
    fn kperf_reset() -> c_int;
    fn kperf_ns_to_ticks(ns: c_ulonglong) -> c_ulonglong;
    fn kperf_ticks_to_ns(ticks: c_ulonglong) -> c_ulonglong;
    fn kperf_tick_frequency() -> c_ulonglong;
}

#[link(name="kperf-data", kind="framework")]
extern "C" {
    /// Create a config.
    /// @param db A kpep db, see kpep_db_create()
    /// @param cfg_ptr A pointer to receive the new config.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_create (db: *mut kpep_db, cfg_ptr: *mut kpep_config) -> c_int;

    /// Free the config.
    fn kpep_config_free (cfg: *mut kpep_config);

    /// Add an event to config.
    /// @param cfg The config.
    /// @param ev_ptr A event pointer.
    /// @param flag 0: all, 1: user space only
    /// @param err Error bitmap pointer, can be NULL.
    ///            If return value is `CONFLICTING_EVENTS`, this bitmap contains
    ///            the conflicted event indices, e.g. "1 << 2" means index 2.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_add_event (cfg: *mut kpep_config, ev_ptr: **mut kpep_event, flag: c_uint, err: *mut c_uint) -> c_int;

    /// Remove event at index.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_remove_event (cfg: *mut kpep_config, idx: size_t) -> c_int;

    /// Force all counters.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_force_counters (cfg: *mut kpep_config) -> c_int;

    /// Get events count.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_events_count (cfg: *mut kpep_config, count_ptr: *mut size_t) -> c_int;

    /// Get all event pointers.
    /// @param buf A buffer to receive event pointers.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_events_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_events (cfg: *mut kpep_config, buf: **mut kpep_event, buf_size: size_t) -> c_int;

    /// Get kpc register configs.
    /// @param buf A buffer to receive kpc register configs.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_kpc_count() * sizeof(kpc_config_t).
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_kpc (cfg: *mut kpep_config, buf: *mut kpc_config_t, buf_size: size_t) -> c_int;

    /// Get kpc register config count.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_kpc_count (cfg: *mut kpep_config, count_ptr: *mut size_t) -> c_int;

    /// Get kpc classes.
    /// @param classes See `class mask constants` above.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_kpc_classes (cfg: *mut kpep_config, classes_ptr: *mut c_uint) -> c_int;

    /// Get the index mapping from event to counter.
    /// @param buf A buffer to receive indexes.
    /// @param buf_size The buffer's size in bytes, should not smaller than
    ///                 kpep_config_events_count() * sizeof(kpc_config_t).
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_config_kpc_map (cfg: *mut kpep_config, buf: *mut size_t, buf_size: size_t) -> c_int;

    /// Open a kpep database file in "/usr/share/kpep/" or "/usr/local/share/kpep/".
    /// @param name File name, for example "haswell", "cpu_100000c_1_92fb37c8".
    ///             Pass NULL for current CPU.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_create (name: *const c_char, db_ptr: **mut kpep_db) -> c_int;

    /// Free the kpep database.
    fn kpep_db_free (db: *mut kpep_db);

    /// Get the database's name.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_name (db: *mut kpep_db, name: **const c_char) -> c_int;

    /// Get the event alias count.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_aliases_count (db: *mut kpep_db, count: *mut size_t) -> c_int;

    /// Get all alias.
    /// @param buf A buffer to receive all alias strings.
    /// @param buf_size The buffer's size in bytes,
    ///        should not smaller than kpep_db_aliases_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_aliases (db: *mut kpep_db, buf: **const c_char, buf_size: size_t) -> c_int;

    /// Get counters count for given classes.
    /// @param classes 1: Fixed, 2: Configurable.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_counters_count (db: *mut kpep_db, classes: c_uchar, count: *mut size_t) -> c_int;

    /// Get all event count.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_events_count (db: *mut kpep_db, count: *mut size_t) -> c_int;

    /// Get all events.
    /// @param buf A buffer to receive all event pointers.
    /// @param buf_size The buffer's size in bytes,
    ///        should not smaller than kpep_db_events_count() * sizeof(void *).
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_events (db: *mut kpep_db, buf: **mut kpep_event, buf_size: size_t) -> c_int;

    /// Get one event by name.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_db_event (db: *mut kpep_db, name: *const c_char, ev_ptr: **mut kpep_event) -> c_int;

    /// Get event's name.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_event_name (ev: *mut kpep_event, name_ptr: **const c_char) -> c_int;

    /// Get event's alias.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_event_alias (ev: *mut kpep_event, alias_ptr: **const c_char) -> c_int;

    /// Get event's description.
    /// @return kpep_config_error_code, 0 for success.
    fn kpep_event_description (ev: *mut kpep_event, str_ptr: **const c_char) -> c_int;
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
