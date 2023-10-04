use std::{collections::BinaryHeap, time::Instant};

use rustc_data_structures::fx::FxHashMap;
use serde::{Deserialize, Serialize};
use serde_json::from_str;

use mimalloc::MiMalloc;
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Serialize, Deserialize)]
struct Post {
    _id: String,
    title: String,
    // #[serde(skip_serializing)]
    tags: Vec<String>,
}

const NUM_TOP_ITEMS: usize = 5;

#[derive(Serialize)]
struct RelatedPosts<'a> {
    _id: &'a String,
    tags: &'a Vec<String>,
    related: Vec<&'a Post>,
}

#[derive(Eq)]
struct PostCount {
    post: usize,
    count: usize,
}

impl std::cmp::PartialEq for PostCount {
    fn eq(&self, other: &Self) -> bool {
        self.count == other.count
    }
}

impl std::cmp::PartialOrd for PostCount {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for PostCount {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // reverse order
        other.count.cmp(&self.count)
    }
}

fn least_n<T: Ord>(n: usize, mut from: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    let mut h = BinaryHeap::from_iter(from.by_ref().take(n));

    for it in from {
        // heap thinks the smallest is the greatest because of reverse order
        let mut greatest = h.peek_mut().unwrap();

        if it < *greatest {
            // heap rebalances after the smart pointer is dropped
            *greatest = it;
        }
    }
    h.into_iter()
}

fn main() {
    let start_io_serde = Instant::now();
    let json_str = std::fs::read_to_string("../posts.json").unwrap();
    let start_serde = Instant::now();
    let posts: Vec<Post> = from_str(&json_str).unwrap();

    let start = Instant::now();

    let mut post_tags_map: FxHashMap<&String, Vec<usize>> = FxHashMap::default();

    for (i, post) in posts.iter().enumerate() {
        for tag in &post.tags {
            post_tags_map.entry(tag).or_default().push(i);
        }
    }

    let related_posts: Vec<RelatedPosts<'_>> = posts
        .iter()
        .enumerate()
        .map(|(idx, post)| {
            // faster than allocating outside the loop
            let mut tagged_post_count = vec![0; posts.len()];

            for tag in &post.tags {
                if let Some(tag_posts) = post_tags_map.get(tag) {
                    for &other_post_idx in tag_posts {
                        tagged_post_count[other_post_idx] += 1;
                    }
                }
            }

            tagged_post_count[idx] = 0; // don't recommend the same post

            let top = least_n(
                NUM_TOP_ITEMS,
                tagged_post_count
                    .iter()
                    .enumerate()
                    .map(|(post, &count)| PostCount { post, count }),
            );
            let related = top.map(|it| &posts[it.post]).collect();

            RelatedPosts {
                _id: &post._id,
                tags: &post.tags,
                related,
            }
        })
        .collect();

    let end = Instant::now();

    let duration = end.duration_since(start);
    let duration_serde = end.duration_since(start_serde);
    let duration_io_serde = end.duration_since(start_io_serde);
    
    let serde_overhead = duration_serde - duration;
    let io_overhead = duration_io_serde - duration_serde;
    let io_serde_overhead = duration_io_serde - duration;

    print!(
        "Processing time: {:?}\n",
        duration
    );
    print!(
        "Processing time with serde: {:?}\n",
        duration_serde
    );
    print!("Processing time with IO and serde: {:?}\n",
        duration_io_serde
    );

    print!(
        "Overhead serde: {:?}\n",
        serde_overhead
    );
    print!(
        "Overhead IO: {:?}\n",
        io_overhead
    );
    print!("Overhead IO & serde: {:?}\n",
        io_serde_overhead
    );

    let json_str = serde_json::to_string(&related_posts).unwrap();
    std::fs::write("../related_posts_rust.json", json_str).unwrap();
}
