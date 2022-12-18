use std::cell::{Cell, Ref};
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;

#[derive(PartialEq, Eq, Clone, Hash)]
struct Edge {
    to: String,
    cost: usize,
    flow_rate: usize,
}

struct Node {
    flow_rate: usize,
    location: String,
    edges: Vec<String>,
}

type Graph = HashMap<String, Node>;
type RefinedGraph = HashMap<String, Vec<Edge>>;

fn parse_graphs(sensor_lines: &str) -> RefinedGraph {
    let mut graph: Graph = HashMap::new();

    for line in sensor_lines.lines() {
        let split_regex = regex::Regex::new(r"Valve | has flow rate=|; tunnels lead to valves |; tunnel leads to valve ").unwrap();
        let mut elements = split_regex
            .split(line)
            .map(|str| str.trim())
            .filter(|str| !str.is_empty());

        let name: String = elements.next().unwrap().parse().unwrap();
        let flow_rate_str = elements.next().unwrap();
        let flow_rate = flow_rate_str.parse().unwrap();
        let edges = elements.next().unwrap().split(",").map(|str| str.trim().parse().unwrap()).collect();

        println!("Added: {}", name);

        graph.insert(name.clone(), Node {
            flow_rate,
            location: name.clone(),
            edges,
        });

    }

    refine_graph(graph)
}

fn get_paths_from(name: String, graph: &Graph) -> Vec<Edge> {
    let mut visited_nodes: HashMap<String, usize> = HashMap::new();
    let mut paths: VecDeque<Vec<String>> = VecDeque::new();

    paths.push_front(vec![name]);

    while let Some(path) = paths.pop_back() {
        let current = path.last().unwrap();

        for next in graph[current].edges.iter() {
            if !visited_nodes.contains_key(next) {
                let mut new_path = path.clone();
                new_path.push(next.clone());

                visited_nodes.insert(next.clone(), new_path.len());
                paths.push_front(new_path);
            }
        }
    }

    visited_nodes.into_iter().map(|(key, path_len)| Edge {
        to: key.clone(),
        cost: path_len,
        flow_rate: graph[&key].flow_rate
    }).filter(|edge| edge.flow_rate > 0).collect()
}

fn refine_graph(graph: Graph) -> RefinedGraph {
    let mut refined_graph = HashMap::new();

    for key in (&graph).keys() {
        if graph[key].flow_rate != 0 || key == "AA" {
            refined_graph.insert(key.clone(), get_paths_from(key.clone(), &graph).into_iter().filter(|edge| edge.flow_rate != 0).collect());
        }
    }

    refined_graph
}

fn get_path_cost(path: &Vec<Edge>) -> usize {
    path.iter().map(|edge| edge.cost).sum()
}

fn get_path_score(path: &Vec<Edge>, max_cost: usize) -> usize {
    path.iter().fold((0, 0), |(cost, score), edge| (cost + edge.cost, score + (max_cost - (cost + edge.cost)) * edge.flow_rate)).1
}

fn get_all_paths(graph: &RefinedGraph, max_cost: usize, invalid_paths: &Vec<String>) -> Vec<Vec<Edge>>{
    let mut paths = vec![];
    let initial = Edge {
        to: "AA".parse().unwrap(),
        cost: 0,
        flow_rate: 0,
    };
    let mut paths_to_explore = vec![vec![initial]];

    let max_length = graph.len();

    while let Some(path) = paths_to_explore.pop() {
        paths.push(path.clone());

        let last = path.last().unwrap();

        if path.len() < max_length {
            for next in &graph[&last.to] {
                let mut new_path = path.clone();

                if !new_path.iter().map(|edge| edge.to.as_str()).contains(&next.to.as_str()) && get_path_cost(&new_path) + next.cost <= max_cost && !invalid_paths.contains(&next.to){
                    new_path.push(next.clone());
                    paths_to_explore.push(new_path);
                }
            }
        }
    }

    paths
}

fn get_best_path(mut paths: Vec<Vec<Edge>>, max_cost: usize) -> usize {
    paths.iter().map(|path| get_path_score(path, max_cost)).max().unwrap()
}

fn get_combinations_for(paths: Vec<Vec<Edge>>, graph: &RefinedGraph, max_cost: usize) -> usize {
    let amount = paths.len();

    paths
        .into_iter()
        .enumerate()
        .map(|(i, path)| {
            println!("Processed {}/{}", i, amount);
            get_path_score(&path, max_cost) + get_best_path(get_all_paths(graph, max_cost, &path.iter().map(|el| el.to.clone()).collect()), max_cost)
        })
        .max().unwrap()

}

#[cfg(test)]
mod tests {
    use std::{fs};
    use super::*;

    #[test]
    fn simple_test() {
        let refined_graph = parse_graphs(include_str!("day16/test_simple.txt"));

        assert_eq!(get_best_path(get_all_paths(&refined_graph, 30, &vec![]), 30), 1651);

    }

    #[test]
    fn large_test() {
        let refined_graph = parse_graphs(include_str!("day16/test_large.txt"));

        assert_eq!(get_best_path(get_all_paths(&refined_graph, 30, &vec![]), 30), 2253);
    }

    #[test]
    fn simple_test_2() {
        let refined_graph = parse_graphs(include_str!("day16/test_simple.txt"));

        assert_eq!(get_combinations_for(get_all_paths(&refined_graph, 26, &vec![]), &refined_graph, 26), 1707);

    }

    #[test]
    fn large_test_2() {
        let refined_graph = parse_graphs(include_str!("day16/test_large.txt"));

        assert_eq!(get_combinations_for(get_all_paths(&refined_graph, 26, &vec![]), &refined_graph, 26), 2838);
    }
}