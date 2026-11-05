use std::fs;
use std::path::{Path, PathBuf};

pub struct ThumbnailCache {
    cache_dir: PathBuf,
}

impl ThumbnailCache {
    pub fn new<P: AsRef<Path>>(cache_dir: P) -> Self {
        let dir = cache_dir.as_ref().to_path_buf();
        let _ = fs::create_dir_all(&dir);
        Self { cache_dir: dir }
    }

    pub fn get_path_for_url(&self, url: &str) -> PathBuf {
        let filename = format!("{:x}.img", md5_simple(url));
        self.cache_dir.join(filename)
    }

    pub fn get(&self, url: &str) -> Option<Vec<u8>> {
        let path = self.get_path_for_url(url);
        if path.exists() {
            fs::read(path).ok()
        } else {
            None
        }
    }

    pub fn store(&self, url: &str, data: &[u8]) -> std::io::Result<PathBuf> {
        let path = self.get_path_for_url(url);
        fs::write(&path, data)?;
        Ok(path)
    }
}

fn md5_simple(input: &str) -> u64 {
    let mut hash: u64 = 5381;
    for b in input.bytes() {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(b as u64);
    }
    hash
}
