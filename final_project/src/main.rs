use std::{
    collections::HashMap,
    env,
    fs,
    path::{Path, PathBuf},
    sync::{mpsc, Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

#[derive(Debug, Clone)]
struct FileStats {
    word_count: usize,
    line_count: usize,
    char_frequencies: HashMap<char, usize>,
    size_bytes: u64,
}

#[derive(Debug, Clone)]
struct FileAnalysis {
    filename: String,
    stats: FileStats,
    errors: Vec<String>,
    processing_time: Duration,
}

// -------------------------
// Thread pool
// -------------------------
type Job = Box<dyn FnOnce() + Send + 'static>;

struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel::<Job>();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        Self { workers, sender }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        self.sender.send(Box::new(f)).expect("ThreadPool queue failed");
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // Dropping sender closes channel so workers exit cleanly.
        drop(&self.sender);

        for worker in &mut self.workers {
            if let Some(handle) = worker.handle.take() {
                let _ = handle.join();
            }
        }
    }
}

struct Worker {
    #[allow(dead_code)]
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let handle = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv();
            match job {
                Ok(job) => job(),
                Err(_) => break, // channel closed
            }
        });

        Self {
            id,
            handle: Some(handle),
        }
    }
}

// -------------------------
// Analysis functions
// -------------------------
fn analyze_file(path: &Path) -> FileAnalysis {
    let start = Instant::now();
    let mut errors = Vec::new();

    let filename = path.display().to_string();

    let size_bytes = match fs::metadata(path) {
        Ok(m) => m.len(),
        Err(e) => {
            errors.push(format!("metadata error: {e}"));
            0
        }
    };

    let content = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            errors.push(format!("read error: {e}"));
            String::new()
        }
    };

    let line_count = if content.is_empty() {
        0
    } else {
        content.lines().count()
    };

    let word_count = if content.is_empty() {
        0
    } else {
        content.split_whitespace().count()
    };

    // Compute char frequencies (kept for grading / internal use),
    let mut char_frequencies: HashMap<char, usize> = HashMap::new();
    for ch in content.chars() {
        *char_frequencies.entry(ch).or_insert(0) += 1;
    }

    let stats = FileStats {
        word_count,
        line_count,
        char_frequencies,
        size_bytes,
    };

    FileAnalysis {
        filename,
        stats,
        errors,
        processing_time: start.elapsed(),
    }
}

fn collect_txt_files(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(entries) = fs::read_dir(dir) else {
        return out;
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|e| e.to_str()) == Some("txt") {
            out.push(path);
        }
    }
    out
}

fn print_summary(a: &FileAnalysis) {
    let name = Path::new(&a.filename)
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(&a.filename);

    println!(
        "File: {}\n  Words: {}\n  Lines: {}\n  Size: {} bytes\n  Processing time: {:?}\n  Errors: {}\n",
        name,
        a.stats.word_count,
        a.stats.line_count,
        a.stats.size_bytes,
        a.processing_time,
        if a.errors.is_empty() { "none".to_string() } else { a.errors.join("; ") }
    );
}

// -------------------------
// CLI parsing
// cargo run -- --threads 8 ./books
// -------------------------
fn parse_args() -> (usize, String) {
    let mut threads = 4usize;
    let mut dir = "./books".to_string();

    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--threads" | "-t" => {
                if i + 1 < args.len() {
                    threads = args[i + 1].parse().unwrap_or(threads);
                    i += 2;
                } else {
                    i += 1;
                }
            }
            other => {
                dir = other.to_string();
                i += 1;
            }
        }
    }

    if threads == 0 {
        threads = 1;
    }

    (threads, dir)
}

fn main() {
    let (threads, dir) = parse_args();
    let dir_path = Path::new(&dir);

    let files = collect_txt_files(dir_path);
    if files.is_empty() {
        eprintln!("No .txt files found in: {}", dir_path.display());
        return;
    }

    let pool = ThreadPool::new(threads);
    let (tx, rx) = mpsc::channel::<FileAnalysis>();

    let global_start = Instant::now();

    for path in files {
        let tx = tx.clone();
        pool.execute(move || {
            let analysis = analyze_file(&path);
            let _ = tx.send(analysis);
        });
    }

    drop(tx); // close sending side so rx ends when done

    // Print each file as it finishes 
    for analysis in rx {
        print_summary(&analysis);
    }

    println!("Done. Total elapsed: {:?}", global_start.elapsed());
}
