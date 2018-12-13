use std::process::exit;

use day07::{read_edges, Graph, Vertex};

fn main() {
    match read_edges().map(Graph::from_edges).map(toposort) {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(order) => println!("{}", order.into_iter().map(|v| v.0).collect::<String>()),
    }
}

fn toposort(mut g: Graph) -> Vec<Vertex> {
    let mut result = Vec::with_capacity(g.0.len());

    let mut open = g.roots();

    while open.len() > 0 {
        let (idx, vertex) = open
            .iter()
            .enumerate()
            .min_by_key(|(_, v)| *v)
            .map(|(i, &v)| (i, v))
            .unwrap();

        open.remove(idx);
        open.extend(g.remove(&vertex).into_iter());

        result.push(vertex);
    }

    result
}
