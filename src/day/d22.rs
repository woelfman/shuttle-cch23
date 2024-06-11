//! Day 22: Dawn of the day before the day before the final day
//!
//! When Christmas Eve rolls around, it's game time. Santa, decked out in his
//! jolly red suit, strides over to his fully-stacked sleigh. His eyes twinkle
//! as he checks his list one last time on his high-tech sleigh dashboard. The
//! backend has been working like a charm after the recent upgrades.
//!
//! With a crack of his whip and a hearty "Ho ho ho," off they go into the snowy
//! night. The reindeers take off like an orange space rocket, disappearing into
//! the starry sky, on a journey to deliver joy to the world!
//!
//! # Task 1: Leave no gift behind!
//!
//! During a last minute database migration in the gift order database, Santa
//! noticed that a small de-sync happened. One gift order slipped through the
//! cracks and only ended up in one of the database replicas. Since it's already
//! Dec 22nd, Santa tells you we need to recover the lost record immediately. No
//! child must be left without a gift this Christmas!
//!
//! When Santa started extracting all gift order IDs from the database replicas,
//! something got jumbled up and caused them to print in a random order.
//! Great... now we have two long lists of random numbers with just one number
//! differing between them. Santa knows you are good at coding, so he
//! concatenates the two files, scrambles the order again and lets you find the
//! integer without a pair.
//!
//! Make a POST endpoint `/22/integers` that takes in a text body with one
//! positive `u64` integer on each line. All integers appear twice, except for
//! one. Find it and respond with a string consisting of that number of Present
//! emojis (🎁).
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/22/integers \
//!   -H 'Content-Type: text/plain' \
//!   -d '888
//! 77
//! 888
//! 22
//! 77
//! '
//!
//! 🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁🎁
//! ```
//!
//! # Task 2: The Shuttle Rocket
//!
//! When Santa speeds across Earth to deliver presents, he looks up to the skies
//! and sees the Shuttle Rocket taking off to the stars. The crew on the rocket,
//! in their quest to visit the stars at the edge of the CCH23 galaxy, has
//! discovered that some magical portals have opened near every star. The
//! portals allow instant bidirectional travel between stars. This saves the
//! crew a lot of flight time to the outer edge, but now they need to figure out
//! which portals to take in order to get to the destination.
//!
//! The input is sent as text in a POST request to `/22/rocket` in this format:
//!
//! * The first line has a number N (`2 <= N <= 100`), the number of stars in the galaxy.
//! * On the following N lines are the 3D coordinates of each star in the galaxy as three `i32s`.
//! * Then follows a line with the number K (`1 <= K <= 100`), the number of portals in the galaxy.
//! * On the following K lines are the stars that each portal connects as two star indices.
//!
//! The crew wants to travel from star *0* to star *N-1* on the path that goes
//! through the least amount of portals, since going through portals make them
//! feel dizzy.
//!
//! After the path with the least portals has been found, the crew wants to know:
//!
//! * How many portals did they have to go through?
//! * How long would the path they took have been if no portals existed? (as an `f32` rounded to 3 decimals)
//!
//! Remember to not get stuck in an infinite portal loop!
//!
//! ## Example
//!
//! ```not_rust
//! curl -X POST http://localhost:8000/22/rocket \
//!   -H 'Content-Type: text/plain' \
//!   -d '5
//! 0 1 0
//! -2 2 3
//! 3 -3 -5
//! 1 1 5
//! 4 3 5
//! 4
//! 0 1
//! 2 4
//! 3 4
//! 1 2
//! '
//!
//! 3 26.123
//! ```
//!
//! Explanation:
//!
//! There are 5 stars and 4 portals. We can get from star 0 to star 4 by going
//! through these portals:
//!
//! * portal 0 from star 0 to star 1
//! * portal 3 from star 1 to star 2
//! * portal 1 from star 2 to star 4
//!
//! The path is 0 -> 1 -> 2 -> 4. 3 portals were used. The length of this path
//! without taking any portals would have been `distance(star 0, star 1) +
//! distance(star 1, star 2) + distance(star 2, star 4)` where `distance()` is
//! the distance between two stars.
use axum::{routing::post, Router};
use itertools::Itertools;
use petgraph::{algo::astar, stable_graph::NodeIndex, Graph};

pub fn get_routes() -> Router {
    Router::new()
        .route("/22/integers", post(integers))
        .route("/22/rocket", post(rocket))
}

async fn integers(body: String) -> String {
    let mut ints: Vec<u64> = body.lines().filter_map(|n| n.parse().ok()).collect();

    ints.sort_unstable();

    for mut chunk in &ints.into_iter().chunks(2) {
        let a = chunk.next();
        let b = chunk.next();

        if a != b {
            return '🎁'
                .to_string()
                .repeat(usize::try_from(a.unwrap()).unwrap_or(usize::MAX));
        }
    }

    String::new()
}

struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
}

impl Coordinate {
    pub fn distance(&self, b: &Self) -> f32 {
        ((self.x as f32 - b.x as f32).powi(2)
            + (self.y as f32 - b.y as f32).powi(2)
            + (self.z as f32 - b.z as f32).powi(2))
        .sqrt()
    }
}

struct Portal {
    entrance: u8,
    exit: u8,
}

async fn rocket(body: String) -> String {
    let mut graph = Graph::new();
    let mut iter = body.lines();
    let n = iter.next().and_then(|n| n.parse::<u8>().ok()).unwrap_or(0);
    for _ in 0..n {
        let coordinate: Coordinate = iter
            .next()
            .map(|s| s.split_whitespace().filter_map(|s| s.parse::<i32>().ok()))
            .map(|mut v| Coordinate {
                x: v.next().unwrap(),
                y: v.next().unwrap(),
                z: v.next().unwrap(),
            })
            .unwrap();
        graph.add_node(coordinate);
    }
    let k = iter.next().and_then(|k| k.parse::<u8>().ok()).unwrap_or(0);
    for _ in 0..k {
        let portal: Portal = iter
            .next()
            .map(|s| s.split_whitespace().filter_map(|s| s.parse::<u8>().ok()))
            .map(|mut v| Portal {
                entrance: v.next().unwrap(),
                exit: v.next().unwrap(),
            })
            .unwrap();
        graph.add_edge(
            NodeIndex::new(portal.entrance.into()),
            NodeIndex::new(portal.exit.into()),
            (),
        );
    }

    let end = NodeIndex::new(graph.node_count() - 1);
    let path = astar(
        &graph,
        NodeIndex::new(0),
        |finish| finish == end,
        |_| 1,
        |_| 0,
    )
    .unwrap();

    let distance = path
        .1
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|(a, b)| {
            (
                graph.node_weight(*a).unwrap(),
                graph.node_weight(*b).unwrap(),
            )
        })
        .fold(0.0f32, |acc, (a, b)| acc + a.distance(b));

    format!("{} {:.3}", path.0, distance)
}
