mod stat;

use actix_web::http::StatusCode;
use chrono::{DateTime, Timelike, Utc};
use proptest::prelude::*;
use reqwest::Client;
use std::time::Instant;
use tokio::runtime::Runtime;

use crate::{
    envs::vars::get_client_url,
    scheme::posts::{Post, PostInput},
};
use stat::*;

fn truncate_to_micros(dt: DateTime<Utc>) -> DateTime<Utc> {
    dt.with_nanosecond(dt.timestamp_subsec_micros() * 1000)
        .unwrap()
}

// End-to-end property-based test that exercises the full lifecycle of post management.
//
// The test executes the following scenario for a randomly generated batch of posts:
//
// 1. A number of `PostInput` instances are generated randomly.
// 2. Each post is sent to the server via a `POST /posts` request.
// 3. Each created post is then fetched individually via `GET /posts/{id}` and compared to the original input.
// 4. Each post is updated (fields modified) via `PUT /posts/{id}`.
// 5. The full list of posts is fetched via `GET /posts` and each updated post is checked for consistency.
// 6. Each post is deleted via `DELETE /posts/{id}`.
// 7. A final call to `GET /posts` is made to verify that all previously created posts are gone.
//
// ### Performance Metrics:
//
// For each HTTP request made during the test, the elapsed response time is measured and recorded.
// After all operations, basic statistics (e.g., min, max, mean latency) are computed and printed or logged,
// giving a rough indication of the server’s performance across thousands of randomized test iterations.
//
// ### Important Notes:
//
// - **Error handling is intentionally omitted** due to time constraints. In a production-grade test,
//   additional checks for failure scenarios (e.g. 400/404 responses) should be included.
// - **The total number of posts returned from `GET /posts` is not guaranteed to match the number of posts created**.
//   This is because the test is designed to run in parallel with other tests or sessions,
//   so the global server state may contain posts created by other test cases.
//
// This test is useful for validating the correctness of state transitions and API conformance
// under randomized but controlled input data.
//
// # Property-based Testing
//
// Uses the `proptest` crate to generate inputs and assert round-trip consistency.
//
// # Panics
// Will panic if any request fails unexpectedly or if any data mismatch occurs.
proptest! {
    #![proptest_config(ProptestConfig {
        max_shrink_iters: 50,
        ..ProptestConfig::with_cases(1000)
    })]

    #[allow(non_snake_case)]
    #[test]
    fn test(posts in proptest::collection::vec(PostInput::arbitrary(), 100)) {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let client = Client::new();
            let mut measuremnt: Vec<TimeMeasument> = Vec::new();
            let mut times = Vec::new();
            let mut ids = Vec::new();

            // Create posts
            {
                for post in posts.iter() {
                    let start = Instant::now();
                    // Create a post
                    let response = client
                        .post(format!("http://{}/posts", get_client_url()))
                        .header("Authorization", "Bearer fake_test_token")
                        .json(post)
                        .send()
                        .await;
                    // Check network status
                    assert!(response.is_ok(), "request failed: {:?}", response.err());

                    // Check server status
                    let response = response.unwrap();
                    let status = response.status();
                    assert_eq!(status.as_u16(), StatusCode::CREATED, "unexpected status: {status}");
                    times.push(start.elapsed().as_nanos());
                    // println!("Post created in {} ms",start.elapsed().as_millis());
                    // Get a post
                    let published: Post = response.json().await.unwrap();

                    // Check post
                    assert_eq!(post.author, published.author);
                    assert_eq!(post.content, published.content);
                    assert_eq!(truncate_to_micros(post.date), truncate_to_micros(published.date));

                    // Check unique of id
                    assert!(!ids.contains(&published.id));

                    // Save ID
                    ids.push(published.id);
                }
                // Save statistic
                measuremnt.push(TimeMeasument::CreatePost(times));
            }


            {
                // Gettings posts
                let mut times = Vec::new();
                for (idx, id) in ids.iter().enumerate() {
                    let start = Instant::now();
                    // Get a post
                    let response = client
                        .get(format!("http://{}/posts/{id}", get_client_url()))
                        .header("Authorization", "Bearer fake_test_token")
                        .send()
                        .await;
                    // Check network status
                    assert!(response.is_ok(), "request failed: {:?}", response.err());

                    // Check server status
                    let response = response.unwrap();
                    let status = response.status();
                    assert_eq!(status.as_u16(), StatusCode::OK, "unexpected status: {status}");
                    times.push(start.elapsed().as_nanos());
                    // println!("Post gotten in {} ms",start.elapsed().as_millis());
                    // Get a post
                    let post: Post = response.json().await.unwrap();

                    // Check post
                    assert_eq!(post.author, posts[idx].author);
                    assert_eq!(post.content, posts[idx].content);
                    assert_eq!(truncate_to_micros(post.date), truncate_to_micros(posts[idx].date));

                }

                // Save statistic
                measuremnt.push(TimeMeasument::GetPost(times));
            }

            // Updating posts
            {
                let mut times = Vec::new();
                for (idx, id) in ids.iter().enumerate() {
                    let start = Instant::now();
                    // Update a post
                    let response = client
                        .put(format!("http://{}/posts/{id}", get_client_url()))
                        .header("Authorization", "Bearer fake_test_token")
                        .json(&PostInput {  content: "-".to_owned(), author: "-".to_owned(), date: posts[idx].date.to_owned()})
                        .send()
                        .await;
                    // Check network status
                    assert!(response.is_ok(), "request failed: {:?}", response.err());

                    // Check server status
                    let response = response.unwrap();
                    let status = response.status();
                    assert_eq!(status.as_u16(), StatusCode::OK, "unexpected status: {status}");
                    times.push(start.elapsed().as_nanos());
                    // println!("Post updated in {} ms",start.elapsed().as_millis());
                    // Get a post
                    let post: Post = response.json().await.unwrap();

                    // Check post
                    assert_eq!(post.author, "-");
                    assert_eq!(post.content, "-");
                    assert_eq!(truncate_to_micros(post.date), truncate_to_micros(posts[idx].date));

                }

                measuremnt.push(TimeMeasument::UpdatePost(times));
            }


            // Get all posts
            {
                let start = Instant::now();
                let response = client
                    .get(format!("http://{}/posts", get_client_url() ))
                    .header("Authorization", "Bearer fake_test_token")
                    .send()
                    .await;
                // Check network status
                assert!(response.is_ok(), "request failed: {:?}", response.err());

                // Check server status
                let response = response.unwrap();
                let status = response.status();
                assert_eq!(status.as_u16(), StatusCode::OK, "unexpected status: {status}");
                measuremnt.push(TimeMeasument::ListPost(start.elapsed().as_nanos()));
                // println!("Post list is gotten in {} ms",start.elapsed().as_millis());

                // Get a posts list
                let all: Vec<Post> = response.json().await.unwrap();

                for id in ids.iter() {
                    let actual = all.iter().find(|post| &post.id == id).unwrap();
                    assert_eq!(actual.author, "-");
                    assert_eq!(actual.content, "-");
                    assert!(ids.contains(&actual.id));
                }
            }


            // Remove posts
            {
                let mut times = Vec::new();

                for id in ids.iter() {
                    let start = Instant::now();
                    // Remove a post
                    let response = client
                        .delete(format!("http://{}/posts/{id}", get_client_url()))
                        .header("Authorization", "Bearer fake_test_token")
                        .send()
                        .await;
                    // Check network status
                    assert!(response.is_ok(), "request failed: {:?}", response.err());

                    // Check server status
                    let response = response.unwrap();
                    let status = response.status();
                    assert_eq!(status.as_u16(), StatusCode::NO_CONTENT, "unexpected status: {status}");
                    times.push(start.elapsed().as_nanos());
                    // println!("Post deleted in {} ms",start.elapsed().as_millis());

                }

                measuremnt.push(TimeMeasument::DeletePost(times));
            }

            // Get all posts
            {
                let response = client
                    .get(format!("http://{}/posts", get_client_url() ))
                    .header("Authorization", "Bearer fake_test_token")
                    .send()
                    .await;
                // Check network status
                assert!(response.is_ok(), "request failed: {:?}", response.err());

                // Check server status
                let response = response.unwrap();
                let status = response.status();
                assert_eq!(status.as_u16(), StatusCode::OK, "unexpected status: {status}");
                // Get a posts list
                let all: Vec<Post> = response.json().await.unwrap();

                for id in ids.iter() {
                    assert!(!all.iter().any(|post| &post.id == id));
                }
            }

            // Add statistics
            statistics().write().unwrap().append(measuremnt);
            statistics().write().unwrap().report();

        });
    }

}

// #[derive(Debug, Clone)]
// pub struct Request {
//     pub path: String,
//     pub method: String,
//     pub headers: HashMap<String, String>,
//     pub input: Vec<Input>,
//     pub response: StatusCode,
// }

// impl Request {
//     pub fn list_posts() -> Self {
//         Request {
//             path: "/posts".to_owned(),
//             method: "get".to_owned(),
//             headers: HashMap::new(),
//             input: Vec::new(),
//             response: StatusCode::OK,
//         }
//     }
//     pub fn create_post() -> Self {
//         Request {
//             path: "/posts".to_owned(),
//             method: "post".to_owned(),
//             headers: HashMap::new(),
//             input: Vec::new(),
//             response: StatusCode::OK,
//         }
//     }
//     pub fn get_post() -> Self {
//         Request {
//             path: "/posts/{id}".to_owned(),
//             method: "get".to_owned(),
//             headers: HashMap::new(),
//             input: Vec::new(),
//             response: StatusCode::OK,
//         }
//     }
//     pub fn update_post() -> Self {
//         Request {
//             path: "/posts/{id}".to_owned(),
//             method: "put".to_owned(),
//             headers: HashMap::new(),
//             input: Vec::new(),
//             response: StatusCode::NO_CONTENT,
//         }
//     }
//     pub fn delete_post() -> Self {
//         Request {
//             path: "/posts/{id}".to_owned(),
//             method: "delete".to_owned(),
//             headers: HashMap::new(),
//             input: Vec::new(),
//             response: StatusCode::NO_CONTENT,
//         }
//     }
// }

// #[derive(Debug, Clone)]
// pub enum Input {
//     PostInput(PostInput),
// }
