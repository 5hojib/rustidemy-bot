// src/web_cache.rs

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Course {
    pub url: String,
    pub thumbnail: String,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CacheEntry {
    pub course: Course,
    pub timestamp: u64,
}

pub struct WebCache {
    pub entries: HashMap<String, CacheEntry>,
    path: String,
}

impl WebCache {
    pub fn new(path: &str) -> Self {
        let mut cache = WebCache {
            entries: HashMap::new(),
            path: path.to_string(),
        };
        if let Err(e) = cache.load() {
            println!("Could not load cache file, starting with an empty cache: {}", e);
            // If the file doesn't exist, we create it.
            if let Err(e) = cache.save() {
                eprintln!("Failed to create a new cache file: {}", e);
            }
        }
        cache
    }

    fn load(&mut self) -> io::Result<()> {
        if Path::new(&self.path).exists() {
            let data = fs::read_to_string(&self.path)?;
            if !data.is_empty() {
                self.entries = serde_json::from_str(&data)
                    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
            }
        }
        Ok(())
    }

    pub fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(&self.entries)?;
        let mut file = fs::File::create(&self.path)?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }

    pub fn add(&mut self, course: Course) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Use course URL as the key to ensure uniqueness
        let key = course.url.clone();

        let entry = CacheEntry {
            course,
            timestamp: now,
        };

        self.entries.insert(key, entry);
    }

    pub fn prune(&mut self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let two_days_in_seconds = 2 * 24 * 60 * 60;

        self.entries
            .retain(|_, entry| now - entry.timestamp < two_days_in_seconds);
    }

    pub fn get_all_courses(&self) -> Vec<Course> {
        self.entries.values().map(|entry| entry.course.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_ID: AtomicUsize = AtomicUsize::new(0);

    fn get_test_cache_path() -> String {
        let id = TEST_ID.fetch_add(1, Ordering::SeqCst);
        format!("test_cache_{}.json", id)
    }

    fn cleanup_test_cache(path: &str) {
        if Path::new(path).exists() {
            fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn test_add_and_get_courses() {
        let path = get_test_cache_path();
        let mut cache = WebCache::new(&path);
        let course = Course {
            url: "http://example.com/course1".to_string(),
            thumbnail: "http://example.com/thumb1.jpg".to_string(),
            title: "Test Course 1".to_string(),
            description: "Description 1".to_string(),
        };
        cache.add(course.clone());

        let courses = cache.get_all_courses();
        assert_eq!(courses.len(), 1);
        assert_eq!(courses[0].url, course.url);
        cleanup_test_cache(&path);
    }

    #[test]
    fn test_prune() {
        let path = get_test_cache_path();
        let mut cache = WebCache::new(&path);

        let old_course = Course {
            url: "http://example.com/old_course".to_string(),
            thumbnail: "http://example.com/old_thumb.jpg".to_string(),
            title: "Old Course".to_string(),
            description: "Old Description".to_string(),
        };

        // Manually create an old entry
        let mut old_entry = CacheEntry {
            course: old_course,
            timestamp: 0, // A long time ago
        };

        // Get current time and subtract more than two days
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let three_days_ago = now - (3 * 24 * 60 * 60);
        old_entry.timestamp = three_days_ago;

        cache.entries.insert(old_entry.course.url.clone(), old_entry);

        let new_course = Course {
            url: "http://example.com/new_course".to_string(),
            thumbnail: "http://example.com/new_thumb.jpg".to_string(),
            title: "New Course".to_string(),
            description: "New Description".to_string(),
        };
        cache.add(new_course);

        assert_eq!(cache.entries.len(), 2);
        cache.prune();
        assert_eq!(cache.entries.len(), 1);
        assert!(cache.entries.contains_key("http://example.com/new_course"));
        cleanup_test_cache(&path);
    }

    #[test]
    fn test_uniqueness() {
        let path = get_test_cache_path();
        let mut cache = WebCache::new(&path);
        let course1 = Course {
            url: "http://example.com/course1".to_string(),
            thumbnail: "http://example.com/thumb1.jpg".to_string(),
            title: "Test Course 1".to_string(),
            description: "Description 1".to_string(),
        };
        let course2 = Course {
            url: "http://example.com/course1".to_string(), // Same URL
            thumbnail: "http://example.com/thumb2.jpg".to_string(),
            title: "Test Course 2".to_string(),
            description: "Description 2".to_string(),
        };
        cache.add(course1);
        cache.add(course2.clone());

        let courses = cache.get_all_courses();
        assert_eq!(courses.len(), 1);
        assert_eq!(courses[0].title, "Test Course 2");
        cleanup_test_cache(&path);
    }

    #[test]
    fn test_save_and_load() {
        let path = get_test_cache_path();
        let mut cache = WebCache::new(&path);
        let course = Course {
            url: "http://example.com/course1".to_string(),
            thumbnail: "http://example.com/thumb1.jpg".to_string(),
            title: "Test Course 1".to_string(),
            description: "Description 1".to_string(),
        };
        cache.add(course.clone());
        cache.save().unwrap();

        let mut new_cache = WebCache::new(&path);
        let _ = new_cache.load(); // This is called in `new`, but we call it again for clarity.
        let courses = new_cache.get_all_courses();
        assert_eq!(courses.len(), 1);
        assert_eq!(courses[0].url, course.url);
        cleanup_test_cache(&path);
    }
}
