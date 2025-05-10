use std::fs;
use std::path::Path;
use std::time::Instant;

fn main() {
    let mut count: u32 = 0;
    let root_path = "D:\\";
    let start = Instant::now();
    scan_directory(root_path, &mut count);
    let duration = start.elapsed();
    println!("总计{}个文件，用时{} ms", count, duration.as_millis());
}

// 相比Java的walkFileTree还要慢一倍
fn scan_directory(dir: &str, count: &mut u32) {
    let entries = fs::read_dir(dir).ok();
    if let Some(read_dir) = entries {
        for entry in read_dir {
            if let Ok(entry) = entry {
                if let Ok(filetype) = entry.file_type() {
                    if filetype.is_dir() {
                        let base_dir = Path::new(dir).canonicalize().unwrap();
                        let filename = entry.file_name();
                        let sub_directory_path = base_dir.join(&filename);
                        scan_directory(sub_directory_path.to_str().unwrap(), count);
                    } else {
                        *count += 1;
                    }
                }
            }
        }
    }
}
