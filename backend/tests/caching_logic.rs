
use std::sync::{Arc, Mutex};
use chrono::{Utc, Duration};
use backend::{Course, fetch_and_update_cache};

#[tokio::test]
async fn test_cache_pruning() {
    let cache = Arc::new(Mutex::new(vec![
        Course {
            id: "1".to_string(),
            title: "Old Course".to_string(),
            description: "".to_string(),
            thumbnail: "".to_string(),
            link: "old_link".to_string(),
            category: None,
            high_res_cover: None,
            resolved_udemy_url: None,
            created_at: Utc::now() - Duration::days(3),
        },
        Course {
            id: "2".to_string(),
            title: "New Course".to_string(),
            description: "".to_string(),
            thumbnail: "".to_string(),
            link: "new_link".to_string(),
            category: None,
            high_res_cover: None,
            resolved_udemy_url: None,
            created_at: Utc::now(),
        },
    ]));

    // Clear the cache before running the test to ensure a clean state
    cache.lock().unwrap().clear();

    cache.lock().unwrap().push(Course {
        id: "1".to_string(),
        title: "Old Course".to_string(),
        description: "".to_string(),
        thumbnail: "".to_string(),
        link: "old_link".to_string(),
        category: None,
        high_res_cover: None,
        resolved_udemy_url: None,
        created_at: Utc::now() - Duration::days(3),
    });

    cache.lock().unwrap().push(Course {
        id: "2".to_string(),
        title: "New Course".to_string(),
        description: "".to_string(),
        thumbnail: "".to_string(),
        link: "new_link".to_string(),
        category: None,
        high_res_cover: None,
        resolved_udemy_url: None,
        created_at: Utc::now(),
    });

    let _ = fetch_and_update_cache(cache.clone()).await;

    let courses = cache.lock().unwrap();
    assert!(courses.len() > 1, "Cache should have more than one course after fetching.");
    assert!(courses.iter().any(|c| c.title == "New Course"), "Cache should contain the new course.");
    assert!(!courses.iter().any(|c| c.title == "Old Course"), "Cache should not contain the old course.");
}
