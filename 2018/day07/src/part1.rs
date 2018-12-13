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

    let mut open =
        g.0.iter()
            .map(|(v, _)| *v)
            .filter(|v| g.incoming(*v).map(|_| 1).fold(0, Add::add) == 0)
            .collect::<Vec<Vertex>>();

    while open.len() > 0 {
        let (idx, vertex) = open
            .iter()
            .enumerate()
            .min_by_key(|(_, v)| *v)
            .map(|(i, &v)| (i, v))
            .unwrap();
        open.remove(idx);

        let outgoing = g.0.remove(&vertex).unwrap();

        for target in outgoing {
            if g.incoming(target).map(|_| 1).fold(0, Add::add) == 0 {
                open.push(target);
            }
        }

        result.push(vertex);
    }

    result
}
