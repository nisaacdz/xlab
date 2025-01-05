use std::{path::PathBuf, sync::OnceLock};

pub static CACHE_DIR: OnceLock<PathBuf> = OnceLock::new();
pub static DATA_DIR: OnceLock<PathBuf> = OnceLock::new();

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use record::{record, save_video, stop};
    use user::{update_frame_rate, update_pointer};

    use super::*;

    #[test]
    fn test_generate_random_string() {
        let cache_dir = PathBuf::from("cache");
        let data_dir = PathBuf::from("output");
        CACHE_DIR.set(cache_dir).ok();
        DATA_DIR.set(data_dir).ok();
        update_pointer(1);
        update_frame_rate(24);
        record();
        std::thread::sleep(Duration::from_secs(12));
        stop();
        super::record::get_record_handle().lock().unwrap().take().map(|u| u.join());
        save_video();
        super::record::get_save_handle().lock().unwrap().take().map(|v| v.join());
    }
}

pub mod options;
pub mod record;
pub mod user;
pub mod video;