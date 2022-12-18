use std::collections::{BTreeSet, HashMap};

use crate::solvers::error::InputParseError;
use lazy_static::lazy_static;

use super::{base::AocSolver, day01::TopK};
use regex::Regex;

type NodeId = u8;

#[derive(Copy, Clone, Debug)]
struct Node {
    flow_rate: u64,
}

#[derive(Clone, Debug)]
struct Graph {
    edges: HashMap<NodeId, Vec<(u64, NodeId)>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, weight: u64, from: NodeId, to: NodeId) {
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.push((weight, to));
        } else {
            self.edges.insert(from, vec![(weight, to)]);
        }
    }

    pub fn disolve_node(&mut self, node: NodeId) {
        if let Some(edges) = self.edges.remove(&node) {
            for (i, e1) in edges.iter().enumerate() {
                if let Some(to_remove) = self
                    .edges
                    .get(&e1.1)
                    .expect("back edges")
                    .iter()
                    .position(|e| e.1 == node)
                {
                    self.edges.get_mut(&e1.1).unwrap().remove(to_remove);
                }
                for e2 in &edges[i + 1..] {
                    self.add_edge(e1.0 + e2.0, e1.1, e2.1);
                    self.add_edge(e1.0 + e2.0, e2.1, e1.1);
                }
            }
        }
    }

    pub fn neighbours(&self, node: NodeId) -> &[(u64, NodeId)] {
        lazy_static! {
            static ref EMPTY: Vec<(u64, NodeId)> = vec![];
        }
        self.edges.get(&node).unwrap_or(&EMPTY)
    }
}

struct NodeIdMap(HashMap<[u8; 2], u8>);

impl NodeIdMap {
    fn new() -> Self {
        Self(HashMap::new())
    }

    fn convert(&mut self, value: [u8; 2]) -> u8 {
        if let Some(&idx) = self.0.get(&value) {
            idx
        } else {
            let idx = self.0.len() as u8;
            self.0.insert(value, idx);
            idx
        }
    }
}

pub struct Solver {
    nodes: Vec<Node>,
    graph: Graph,
    start: NodeId,
}

impl<'a> AocSolver<'a, u64, u64> for Solver {
    fn new(input: &'a str) -> anyhow::Result<Self>
    where
        Self: Sized,
    {
        let re: Regex =
            Regex::new(r"^Valve ([A-Z]{2}) has flow rate=(\d+); tunnels? leads? to valves? (.*)$")
                .unwrap();
        let mut nodes = HashMap::new();
        let mut graph = Graph::new();
        let mut node_ids = NodeIdMap::new();
        for line in input.split('\n') {
            if line.trim().is_empty() {
                continue;
            }
            let mut captures = re.captures_iter(line);
            let matched = captures
                .next()
                .ok_or_else(|| InputParseError::new("invalid format".into()))?;

            let node_id = node_ids.convert([matched[1].as_bytes()[0], matched[1].as_bytes()[1]]);
            nodes.insert(
                node_id,
                Node {
                    flow_rate: matched[2].parse()?,
                },
            );
            for edge in matched[3].split(", ") {
                let to = edge.trim().as_bytes();
                graph.add_edge(1, node_id, node_ids.convert([to[0], to[1]]));
            }
        }
        let start = node_ids.convert([b'A', b'A']);
        for (node_id, node) in nodes.iter() {
            if *node_id != start && node.flow_rate == 0 {
                graph.disolve_node(*node_id)
            }
        }
        Ok(Self {
            nodes: (0..nodes.len())
                .map(|i| *nodes.get(&(i as u8)).unwrap())
                .collect(),
            graph,
            start,
        })
    }

    fn solve_part1(&self) -> anyhow::Result<u64> {
        let mut dp_max_flow = DpMaxFlow::new(&self.nodes, &self.graph);
        let result = dp_max_flow.max_flow(30, self.start, &mut BTreeSet::new());

        Ok(result)
    }

    fn solve_part2(&self) -> anyhow::Result<Option<u64>> {
        let mut dp_max_flow = DpMaxFlowElephant::new(&self.nodes, &self.graph);
        let result = dp_max_flow.max_flow(26, 26, self.start, self.start, 0);
        Ok(Some(result))
    }
}

struct DpMaxFlow<'a> {
    cache: HashMap<(usize, NodeId, BTreeSet<NodeId>), u64>,
    nodes: &'a [Node],
    graph: &'a Graph,
    num_non_zero_flow_rates: usize,
}

impl<'a> DpMaxFlow<'a> {
    fn new(nodes: &'a [Node], graph: &'a Graph) -> Self {
        Self {
            cache: HashMap::new(),
            nodes,
            graph,
            num_non_zero_flow_rates: nodes.iter().filter(|node| node.flow_rate > 0).count(),
        }
    }

    pub fn max_flow(
        &mut self,
        steps_left: usize,
        node_id: NodeId,
        opened_valves: &mut BTreeSet<NodeId>,
    ) -> u64 {
        if steps_left <= 1 {
            return 0;
        }
        if opened_valves.len() >= self.num_non_zero_flow_rates {
            return 0;
        }

        let cache_key = (steps_left, node_id, opened_valves.clone());
        if let Some(&result) = self.cache.get(&cache_key) {
            return result;
        }

        let mut best = TopK::<u64, 1>::new();

        let node = &self.nodes[node_id as usize];
        if node.flow_rate > 0 && !opened_valves.contains(&node_id) {
            opened_valves.insert(node_id);
            best.push(
                node.flow_rate * (steps_left - 1) as u64
                    + self.max_flow(steps_left - 1, node_id, opened_valves),
            );
            opened_valves.remove(&node_id);
        }

        for &(dist, neighbour) in self.graph.neighbours(node_id) {
            if (dist as usize) < steps_left {
                best.push(self.max_flow(steps_left - dist as usize, neighbour, opened_valves))
            }
        }

        let result = best.peek().copied().unwrap_or_default();
        self.cache.insert(cache_key, result);
        result
    }
}

struct DpMaxFlowElephant<'a> {
    cache: HashMap<(usize, usize, NodeId, NodeId, u32), u64>,
    nodes: &'a [Node],
    graph: &'a Graph,
    num_non_zero_flow_rates: usize,
}

impl<'a> DpMaxFlowElephant<'a> {
    fn new(nodes: &'a [Node], graph: &'a Graph) -> Self {
        Self {
            cache: HashMap::new(),
            nodes,
            graph,
            num_non_zero_flow_rates: nodes.iter().filter(|node| node.flow_rate > 0).count(),
        }
    }

    pub fn max_flow(
        &mut self,
        steps_left: usize,
        steps_left_elephant: usize,
        node_id: NodeId,
        elephant_node_id: NodeId,
        opened_valves: u32,
    ) -> u64 {
        if steps_left <= 1 && steps_left_elephant <= 1 {
            return 0;
        }
        if opened_valves.count_ones() as usize >= self.num_non_zero_flow_rates {
            return 0;
        }

        let cache_key = if steps_left <= steps_left_elephant {
            (
                steps_left,
                steps_left_elephant,
                node_id,
                elephant_node_id,
                opened_valves,
            )
        } else {
            (
                steps_left_elephant,
                steps_left,
                elephant_node_id,
                node_id,
                opened_valves,
            )
        };
        if let Some(&result) = self.cache.get(&cache_key) {
            return result;
        }

        let best = if steps_left <= steps_left_elephant && steps_left > 1 {
            self.step_human(
                steps_left,
                steps_left_elephant,
                node_id,
                elephant_node_id,
                opened_valves,
            )
        } else if steps_left_elephant > 1 {
            self.step_elephant(
                steps_left,
                steps_left_elephant,
                node_id,
                elephant_node_id,
                opened_valves,
            )
        } else {
            0
        };

        self.cache.insert(cache_key, best);
        best
    }

    fn step_human(
        &mut self,
        steps_left: usize,
        steps_left_elephant: usize,
        node_id: NodeId,
        elephant_node_id: NodeId,
        opened_valves: u32,
    ) -> u64 {
        let mut best = TopK::<u64, 1>::new();

        let node = &self.nodes[node_id as usize];
        if node.flow_rate > 0 && opened_valves & (1 << node_id) == 0 {
            let opened_valves = opened_valves | (1 << node_id);
            best.push(
                node.flow_rate * (steps_left - 1) as u64
                    + self.max_flow(
                        steps_left - 1,
                        steps_left_elephant,
                        node_id,
                        elephant_node_id,
                        opened_valves,
                    ),
            );
        }

        for &(dist, neighbour) in self.graph.neighbours(node_id) {
            if (dist as usize) < steps_left {
                best.push(self.max_flow(
                    steps_left - dist as usize,
                    steps_left_elephant,
                    neighbour,
                    elephant_node_id,
                    opened_valves,
                ));
            }
        }

        best.peek().copied().unwrap_or_default()
    }

    fn step_elephant(
        &mut self,
        steps_left: usize,
        steps_left_elephant: usize,
        node_id: NodeId,
        elephant_node_id: NodeId,
        opened_valves: u32,
    ) -> u64 {
        let mut best = TopK::<u64, 1>::new();

        let node = &self.nodes[elephant_node_id as usize];
        if node.flow_rate > 0 && opened_valves & (1 << elephant_node_id) == 0 {
            let opened_valves = opened_valves | (1 << elephant_node_id);
            best.push(
                node.flow_rate * (steps_left_elephant - 1) as u64
                    + self.max_flow(
                        steps_left,
                        steps_left_elephant - 1,
                        node_id,
                        elephant_node_id,
                        opened_valves,
                    ),
            );
        }

        for &(dist, neighbour) in self.graph.neighbours(elephant_node_id) {
            if (dist as usize) < steps_left_elephant {
                best.push(self.max_flow(
                    steps_left,
                    steps_left_elephant - dist as usize,
                    node_id,
                    neighbour,
                    opened_valves,
                ));
            }
        }

        best.peek().copied().unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::test::test_example_input;

    #[test]
    fn test_example() {
        let input = include_str!("examples/day16");
        test_example_input::<Solver, _, _>(input, 1651, Some(1707));
    }
}
