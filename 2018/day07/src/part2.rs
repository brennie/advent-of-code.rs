use std::process::exit;

use day07::{read_edges, Graph, Vertex};

fn main() {
    match read_edges()
        .map(Graph::from_edges)
        .map(|g| solve_tasks(g, 5, 60))
    {
        Err(e) => {
            eprintln!("error: {}", e);
            exit(1);
        }

        Ok(time) => println!("{}", time),
    }
}

#[derive(Copy, Clone, Debug)]
struct Task {
    vertex: Vertex,
    finish_at: usize,
}

impl Task {
    fn new(vertex: Vertex, now: usize, penalty: usize) -> Task {
        Task {
            vertex: vertex,
            finish_at: now + penalty + (vertex.0 as usize) - (b'A' as usize) + 1,
        }
    }
}

fn solve_tasks(mut g: Graph, worker_count: usize, penalty: usize) -> usize {
    let mut workers: Vec<Option<Task>> = vec![None; worker_count];

    let mut open = g.roots();

    for time in 0.. {
        for worker in &mut workers {
            if let Some(task) = *worker {
                if task.finish_at == time {
                    *worker = None;

                    open.extend(g.remove(&task.vertex).into_iter());
                }
            }
        }

        'assign: for worker in &mut workers {
            if worker.is_none() {
                if let Some(v) = take_next(&mut open) {
                    *worker = Some(Task::new(v, time, penalty));
                } else {
                    break 'assign;
                }
            }
        }

        if open.len() == 0 && workers.iter().all(Option::is_none) {
            return time;
        }
    }

    unreachable!()
}

fn take_next(open: &mut Vec<Vertex>) -> Option<Vertex> {
    if let Some((i, v)) = open.iter().enumerate().min_by_key(|(_, v)| *v) {
        let v = v.clone();
        open.remove(i);
        Some(v)
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use day07::Edge;

    use super::*;

    #[test]
    fn sample() {
        let g = Graph::from_edges(vec![
            Edge {
                from: Vertex('C'),
                to: Vertex('A'),
            },
            Edge {
                from: Vertex('C'),
                to: Vertex('F'),
            },
            Edge {
                from: Vertex('A'),
                to: Vertex('B'),
            },
            Edge {
                from: Vertex('A'),
                to: Vertex('D'),
            },
            Edge {
                from: Vertex('B'),
                to: Vertex('E'),
            },
            Edge {
                from: Vertex('D'),
                to: Vertex('E'),
            },
            Edge {
                from: Vertex('F'),
                to: Vertex('E'),
            },
        ]);

        assert_eq!(solve_tasks(g, 2, 0), 15);
    }

}
