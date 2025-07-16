use std::{
    env,
    fs::File,
    io::Write,
    sync::{OnceLock, RwLock},
};

use chrono::Utc;

use crate::envs;

/// Enum representing measured response times for different types of operations in the post lifecycle.
///
/// Each variant holds timing data (in nanoseconds) for a particular kind of API request.
/// The data is collected during the test run and used to compute aggregated performance statistics.
#[allow(clippy::enum_variant_names)]
pub enum TimeMeasument {
    /// Response times for all `POST /posts` operations (in nanoseconds).
    CreatePost(Vec<u128>),

    /// Response times for all `GET /posts/{id}` operations.
    GetPost(Vec<u128>),

    /// Response times for all `PUT /posts/{id}` operations.
    UpdatePost(Vec<u128>),

    /// Response time for a single `GET /posts` request (list all posts).
    ListPost(u128),

    /// Response times for all `DELETE /posts/{id}` operations.
    DeletePost(Vec<u128>),
}

/// Aggregated metrics for a single operation type (e.g., Create, Update, Delete).
///
/// This structure keeps track of the number of measured operations, their total execution time,
/// and the average latency. It is used to build a summarized performance report.
#[derive(Default)]
pub struct TestCase {
    count: usize,
    total_time: u128,
    avg_time: u128,
    alias: String,
}

impl TestCase {
    /// Creates a new named `TestCase`.
    ///
    /// # Parameters
    /// - `alias`: A human-readable name for the operation (e.g., `"CreatePost"`).
    pub fn new(alias: String) -> Self {
        Self {
            alias,
            ..Default::default()
        }
    }

    /// Adds multiple measurements to the test case.
    pub fn update_from_times(&mut self, times: &[u128]) {
        self.count += times.len();
        self.total_time += times.iter().sum::<u128>();
    }

    /// Adds a single measurement to the test case.
    pub fn update_from_time(&mut self, time: &u128) {
        self.count += 1;
        self.total_time += time;
    }

    /// Calculates the average latency (`avg_time`) based on the total and count.
    pub fn calc(&mut self) {
        self.avg_time = self.total_time / self.count as u128;
    }
}

/// Collection of all time measurements accumulated across a test run.
///
/// This structure collects all timing data and produces a human-readable summary
/// of performance characteristics per operation type.
#[derive(Default)]
pub struct Statistics {
    times: Vec<TimeMeasument>,
    file: Option<File>,
}

impl Statistics {
    /// Appends a new batch of measurements to the global statistics.
    pub fn append(&mut self, mut times: Vec<TimeMeasument>) {
        self.times.append(&mut times);
    }

    /// Prints a performance report, showing total and average latencies per operation.
    ///
    /// Latency is printed in both nanoseconds and milliseconds for easier interpretation.
    pub fn report(&mut self) {
        let mut create_post = TestCase::new("CreatePost".to_owned());
        let mut get_post = TestCase::new("GetPost".to_owned());
        let mut update_post = TestCase::new("UpdatePost".to_owned());
        let mut list_post = TestCase::new("ListPost".to_owned());
        let mut delete_post = TestCase::new("DeletePost".to_owned());
        for case in self.times.iter() {
            match case {
                TimeMeasument::CreatePost(times) => {
                    create_post.update_from_times(times);
                }
                TimeMeasument::GetPost(times) => {
                    get_post.update_from_times(times);
                }
                TimeMeasument::UpdatePost(times) => {
                    update_post.update_from_times(times);
                }
                TimeMeasument::ListPost(time) => {
                    list_post.update_from_time(time);
                }
                TimeMeasument::DeletePost(times) => {
                    delete_post.update_from_times(times);
                }
            }
        }
        create_post.calc();
        get_post.calc();
        update_post.calc();
        list_post.calc();
        delete_post.calc();
        println!("\n=== Performance Report ===\n");
        println!(
            "{:<15} | {:>10} | {:>12} | {:>10} | {:>12} | {:>10}",
            "Operation", "Count", "Total (ns)", "Avg (ns)", "Total (ms)", "Avg (ms)"
        );
        println!("{}", "-".repeat(80));

        for tc in [
            &create_post,
            &get_post,
            &update_post,
            &list_post,
            &delete_post,
        ] {
            let total_ms = tc.total_time as f64 / 1_000_000.0;
            let avg_ms = tc.avg_time as f64 / 1_000_000.0;

            println!(
                "{:<15} | {:>10} | {:>12} | {:>10} | {:>12.2} | {:>10.2}",
                tc.alias, tc.count, tc.total_time, tc.avg_time, total_ms, avg_ms
            );
        }
        println!("\n");
        self.write(vec![
            create_post.avg_time as f64 / 1_000_000.0,
            get_post.avg_time as f64 / 1_000_000.0,
            update_post.avg_time as f64 / 1_000_000.0,
            list_post.avg_time as f64 / 1_000_000.0,
            delete_post.avg_time as f64 / 1_000_000.0,
        ]);
    }

    fn write(&mut self, row: Vec<f64>) {
        if !envs::vars::write_test_data() {
            return;
        }
        let mut file = if let Some(file) = self.file.take() {
            file
        } else {
            let filename =
                env::temp_dir().join(format!("{}.csv", Utc::now().timestamp().to_string()));
            File::create(filename).expect("Stat data file has been created")
        };
        file.write_all(
            format!(
                "{}\n",
                row.into_iter()
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
                    .join(",")
            )
            .as_bytes(),
        )
        .expect("Stat data hs been written");
        file.flush().expect("Stat data hs been flushed");
        self.file = Some(file);
    }
}

/// Returns a singleton instance of the shared `Statistics` object.
///
/// Internally uses a `OnceLock<RwLock<Statistics>>` to provide thread-safe global access.
/// All test cases and threads share this same instance when collecting metrics.
pub fn statistics() -> &'static RwLock<Statistics> {
    static HASHMAP: OnceLock<RwLock<Statistics>> = OnceLock::new();
    HASHMAP.get_or_init(|| RwLock::new(Statistics::default()))
}
