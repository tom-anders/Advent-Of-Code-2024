use std::cmp::Reverse;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

use priority_queue::PriorityQueue;

pub type Cost = usize;

pub trait Node: Clone + Debug + Eq + Ord + Hash {}
impl<T> Node for T where T: Clone + Debug + Eq + Ord + Hash {}

pub trait UnweightedGraph {
    type Node: Node;
    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = Self::Node> + 'a;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BfsResult<N: Node> {
    distance: Option<usize>,
    visited: HashSet<N>,
}

pub fn bfs<N: Node>(
    graph: &impl UnweightedGraph<Node = N>,
    start: impl Into<N>,
    end: impl Into<N>) -> BfsResult<N> {
    bfs_impl(graph, start, Some(end.into()))
}


pub fn floodfill<N: Node>(
    graph: &impl UnweightedGraph<Node = N>,
    start: impl Into<N>) -> HashSet<N> {
    bfs_impl(graph, start, None).visited
}

fn bfs_impl<N: Node>(
    graph: &impl UnweightedGraph<Node = N>,
    start: impl Into<N>,
    end: Option<N>,
) -> BfsResult<N> {
    let start = start.into();

    let mut distance = 0;
    let mut visited: HashSet<N> = HashSet::new();

    let mut next = HashSet::new();
    next.insert(start);

    loop {
        if next.is_empty() {
            return BfsResult { distance: None, visited };
        }
        let mut neighbors = HashSet::new();
        for node in next {
            visited.insert(node.clone());

            if Some(&node) == end.as_ref() {
                return BfsResult { distance: Some(distance), visited };
            }

            for n in graph.neighbors(&node) {
                if !visited.contains(&n) {
                    neighbors.insert(n);
                }
            }
        }
        next = neighbors;
        distance += 1;
    }
}

pub trait WeightedGraph {
    type Node: Node;
    fn neighbors<'a, 'b: 'a>(
        &'a self,
        node: &'b Self::Node,
    ) -> impl Iterator<Item = (Self::Node, Cost)> + 'a;
}

pub fn dijkstra<T: Node, S: Into<T>>(
    graph: &impl WeightedGraph<Node = T>,
    start_nodes: impl IntoIterator<Item = S>,
    reached_target: impl Fn(&T) -> bool,
) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = PriorityQueue::new();
    for start_node in start_nodes.into_iter() {
        queue.push(start_node.into(), std::cmp::Reverse(0));
    }

    loop {
        match queue.pop() {
            None => return None,
            Some((node, Reverse(current_cost))) => {
                if reached_target(&node) {
                    return Some(current_cost);
                }
                visited.insert(node.clone());

                for (neighbor, cost) in graph.neighbors(&node) {
                    if !visited.contains(&neighbor) && queue.get(&neighbor).is_none() {
                        queue.push(neighbor, Reverse(cost + current_cost));
                    } else if let Some(Reverse(previous_cost)) = queue.get_priority(&neighbor) {
                        let new_cost = current_cost + cost;
                        if new_cost < *previous_cost {
                            queue.change_priority(&neighbor, Reverse(new_cost));
                        }
                    }
                }
            }
        }
    }
}

pub enum NextState<I> {
    Terminal(usize),
    Next(I),
}

pub trait DfsGraph {
    type State: Clone + Debug + Eq + Hash;
    /// Return None for terminal state
    fn next_states<'a, 'b: 'a>(
        &'a self,
        current: &'b Self::State,
        current_best: usize,
        depth: usize,
    ) -> NextState<impl IntoIterator<Item = Self::State> + 'a>;
}

pub fn dfs<T>(graph: &impl DfsGraph<State = T>, start: impl Into<T>) -> Option<Cost> {
    let mut current_best = 0;
    dfs_impl(graph, &start.into(), &mut current_best, 0)
}

fn dfs_impl<T>(
    graph: &impl DfsGraph<State = T>,
    current: &T,
    current_best: &mut Cost,
    depth: usize,
) -> Option<usize> {
    return match graph.next_states(current, *current_best, depth) {
        NextState::Terminal(score) => {
            *current_best = score.max(*current_best);
            Some(*current_best)
        }
        NextState::Next(next_states) => next_states
            .into_iter()
            .flat_map(|next_state| dfs_impl(graph, &next_state, current_best, depth + 1))
            .max(),
    };
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        grid::{self, Grid},
        math::Vec2D,
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[derive(Debug, Clone)]
    struct SimpleWeightedGraph {
        edges: HashMap<&'static str, Vec<(&'static str, usize)>>,
    }

    impl WeightedGraph for SimpleWeightedGraph {
        type Node = &'static str;

        fn neighbors<'a, 'b: 'a>(
            &'a self,
            node: &'b Self::Node,
        ) -> impl Iterator<Item = (Self::Node, Cost)> + 'a {
            self.edges[node].iter().copied()
        }
    }

    #[test]
    fn graph() {
        let mut edges = HashMap::new();
        edges.insert("A", vec![("B", 1), ("D", 2)]);
        edges.insert("B", vec![("C", 7)]);
        edges.insert("C", vec![("F", 1)]);
        edges.insert("D", vec![("C", 3), ("F", 10)]);
        edges.insert("F", vec![]);
        let graph = SimpleWeightedGraph { edges };

        assert_eq!(dijkstra(&graph, ["A"], |&node| node == "B"), Some(1));
        assert_eq!(dijkstra(&graph, ["A"], |&node| node == "D"), Some(2));
        assert_eq!(dijkstra(&graph, ["A"], |&node| node == "C"), Some(5));
        assert_eq!(dijkstra(&graph, ["A"], |&node| node == "F"), Some(6));
        assert_eq!(dijkstra(&graph, ["A"], |&node| node == "G"), None);
    }

    #[test]
    fn grid_diff() {
        impl grid::WeightedGrid for Grid<usize> {
            fn cost(&self, from: Vec2D, to: Vec2D) -> Cost {
                self[to].abs_diff(self[from])
            }

            fn neighbors<'a, 'b: 'a>(
                &'a self,
                node: &'b Vec2D,
            ) -> impl Iterator<Item = Vec2D> + 'a {
                self.orthogonal_neighbors(node)
            }
        }

        let grid: Grid<usize> = vec![
            vec![0, 2, 9, 3, 1, 2],
            vec![1, 9, 1, 3, 3, 3],
            vec![4, 1, 1, 9, 9, 1],
            vec![9, 9, 9, 9, 9, 1],
        ]
        .into();

        assert_eq!(dijkstra(&grid, [Vec2D::new(0, 0)], |&node| node == Vec2D::new(5, 3)), Some(11));
        assert_eq!(dijkstra(&grid, [Vec2D::new(0, 0)], |&node| node == Vec2D::new(-1, -1)), None);
        assert_eq!(dijkstra(&grid, [Vec2D::new(0, 0)], |&node| node == Vec2D::new(0, 0)), Some(0));
    }

    #[test]
    fn grid_bfs() {
        impl grid::UnweightedGrid for Grid<char> {
            fn neighbors<'a, 'b: 'a>(
                &'a self,
                node: &'b Vec2D,
            ) -> impl Iterator<Item = Vec2D> + 'a {
                self.all_neighbors(node).filter(move |pos| self[*pos] != '#')
            }
        }

        let grid: Grid<char> = vec![
            vec!['.', '#', '#', '.', '.', '.', '.'],
            vec!['.', '.', '#', '.', '#', '#', '.'],
            vec!['.', '#', '#', '.', '#', '#', '.'],
            vec!['.', '.', '.', '#', '.', '#', '.'],
        ]
        .into();

        assert_eq!(bfs(&grid, (0, 0), (6, 3)).distance, Some(11));
        assert_eq!(bfs(&grid, (0, 0), (2, 2)).distance, None);
        assert_eq!(bfs(&grid, (0, 0), (0, 0)).distance, Some(0));

        assert_eq!(
            bfs(&grid, (0, 0), (2, 3)),
            BfsResult {
                distance: Some(4),
                visited: [(0, 0), (0, 1), (1, 1), (0, 2), (0, 3), (1, 3), (2, 3)]
                    .into_iter()
                    .map(Vec2D::from)
                    .collect(),
            }
        );
    }

    #[test]
    fn test_dfs() {
        #[derive(Debug, Clone, Eq, PartialEq, Hash)]
        enum State {
            A,
            B,
            C,
            D,
            E,
            F,
            G,
        }

        struct StateGraph;
        impl DfsGraph for StateGraph {
            type State = State;

            fn next_states<'a, 'b: 'a>(
                &'a self,
                current: &'b Self::State,
                _current_best: usize,
                _depth: usize,
            ) -> NextState<impl IntoIterator<Item = Self::State> + 'a> {
                use NextState::*;
                match current {
                    State::A => Next([State::B, State::C]),
                    State::B => Terminal(10),
                    State::C => Next([State::D, State::E]),
                    State::D => Next([State::F, State::G]),
                    State::E => Terminal(11),
                    State::F => Terminal(12),
                    State::G => Terminal(13),
                }
            }
        }

        assert_eq!(dfs(&StateGraph {}, State::A), Some(13)); // A -> C -> H -> 13
    }
}
