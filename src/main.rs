use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
    str::FromStr,
    sync::Arc,
    time::Instant,
};

use clap::{command, Arg, Command};
use eyre::Result;
use futures::future::join_all;
use seahash::hash;
use tokio::{
    fs::{read, read_dir},
    sync::{
        mpsc::{channel, Sender},
        RwLock,
    },
};

#[derive(Debug)]
struct File {
    path: PathBuf,
    size: u64,
}
impl File {
    async fn new(path: &PathBuf, size: u64) -> Self {
        Self {
            path: path.to_path_buf(),
            size,
        }
    }
}

#[derive(Debug)]
struct Dir {
    path: PathBuf,
    depth: u8,
}

async fn get_candidates(path: PathBuf, reccursion_mode: &ReccursionMode, sender: Sender<File>) {
    let dir = Dir {
        path,
        depth: reccursion_mode.depth(),
    };
    let mut tasks = Vec::new();
    let mut dirs_queue = VecDeque::new();
    dirs_queue.push_back(dir);
    let dirs_queue = Arc::new(RwLock::new(dirs_queue));
    loop {
        let dirs_queue = dirs_queue.clone();
        let Some(dir) = dirs_queue.write().await.pop_front() else {
            break;
        };
        let mut entries = match read_dir(&dir.path).await {
            Ok(entries) => entries,
            Err(error) => {
                eprintln!("Failed to read dir {}: {}", dir.path.display(), error);
                continue;
            }
        };
        while let Ok(Some(entry)) = entries.next_entry().await {
            let path = entry.path();
            let metadata = entry.metadata().await.expect("Failed to get metadata");
            if metadata.is_file() && metadata.len() > 0 {
                let sender = sender.clone();
                let handle = tokio::spawn(async move {
                    let file = File::new(&path, metadata.len()).await;
                    sender.send(file).await.expect("Could not process file");
                });
                tasks.push(handle);
            } else if reccursion_mode.is_reccursive() && metadata.is_dir() {
                if !reccursion_mode.is_unlimited() && dir.depth == 0 {
                    continue;
                } else {
                    dirs_queue.write().await.push_back(Dir {
                        path,
                        depth: dir.depth - if reccursion_mode.is_unlimited() { 0 } else { 1 },
                    });
                }
            }
        }
    }
    join_all(tasks).await;
}

#[derive(Debug, Clone)]
enum ReccursionMode {
    None,
    Limited { depth: u8 },
    Unlimited,
}
impl FromStr for ReccursionMode {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        dbg!(input);
        let mode = match input {
            "" => Self::None,
            "unlimited" => Self::Unlimited,
            _ => match input.parse::<u8>() {
                Ok(depth) => Self::Limited { depth },
                Err(_) => Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    "Invalid reccursion depth",
                ))?,
            },
        };
        dbg!(&mode);
        Ok(mode)
    }
}
impl ReccursionMode {
    fn is_reccursive(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }

    fn is_unlimited(&self) -> bool {
        match self {
            Self::Unlimited => true,
            _ => false,
        }
    }

    fn depth(&self) -> u8 {
        match self {
            Self::Limited { depth } => *depth,
            _ => 0,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = command!()
        .arg(Arg::new("path").value_parser(PathBuf::from_str))
        .subcommand(
            Command::new("reccursive")
                .short_flag('r')
                .arg(Arg::new("depth").value_parser(u8::from_str)),
        )
        .get_matches();

    let dir = match matches.get_one::<PathBuf>("path") {
        Some(path) => path.to_path_buf(),
        None => std::env::current_dir()?,
    };
    let reccursion = match matches.subcommand() {
        Some(("reccursive", depth)) => match depth.get_one::<u8>("depth") {
            Some(depth) => ReccursionMode::Limited { depth: *depth },
            None => ReccursionMode::Unlimited,
        },
        _ => ReccursionMode::None,
    };

    let (sender, mut receiver) = channel::<File>(u16::MAX as usize);

    let t = Instant::now();
    get_candidates(dir, &reccursion, sender).await;
    let mut candidates = HashMap::new();
    while let Some(File { path, size }) = receiver.recv().await {
        candidates.entry(size).or_insert_with(Vec::new).push(path);
    }
    candidates.retain(|_, v| v.len() > 1);

    let mut dups = HashMap::new();
    for (_, paths) in candidates {
        for path in paths {
            let content = read(&path).await?;
            let hash = hash(&content);
            dups.entry(hash).or_insert_with(Vec::new).push(path);
        }
    }
    dups.retain(|_, paths| paths.len() > 1);
    let t = t.elapsed();

    if dups.len() == 0 {
        println!("No dups found");
    } else {
        dups.iter().for_each(|(_, paths)| {
            paths.iter().for_each(|path| println!("{}", path.display()));
            println!();
        });
        println!("Found {} dups in {:?}", dups.len(), t);
    }

    Ok(())
}
