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
