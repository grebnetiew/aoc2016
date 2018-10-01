use std::io;
use std::io::BufRead;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::usize;

// Most of this stolen from the docs on BinaryHeap :)

#[derive(Clone, Eq, PartialEq)]
struct ReachedState {
    cost: usize,
    position: State,
}

static mut nChemicals: usize = 0;

#[derive(Clone)]
struct State {
	m: HashMap<Object, usize>
}

impl PartialEq for State {
	fn eq(&self, other: &State) -> bool {
		for i in 0..nChemicals {
			if self.m.get(&Object::Generator(i)) != other.m.get(&Object::Generator(i)) || self.m.get(&Object::Microchip(i)) != other.m.get(&Object::Microchip(i)) {
				return false
			}
		}
		true
	}
}

impl Eq for State {}

impl Ord for State {
	fn cmp(&self, other: &State) -> Ordering {
		// I do not really care about the ordering of the states
		Ordering::Equal
	}
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        None
    }
}

#[derive(PartialEq, Eq, Hash)]
enum Object {
	Generator(usize),
	Microchip(usize),
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for ReachedState {
    fn cmp(&self, other: &ReachedState) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for ReachedState {
    fn partial_cmp(&self, other: &ReachedState) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &Vec<Vec<Edge>>, start: State, goal: State) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(ReachedState { cost: 0, position: start });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(ReachedState { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal { return Some(cost); }

        // Important as we may have already found a better way
        if cost > dist[position] { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = ReachedState { cost: cost + edge.cost, position: edge.node };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}