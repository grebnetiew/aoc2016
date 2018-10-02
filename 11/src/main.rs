use std::hash::Hash;
use std::hash::Hasher;
use std::io;
use std::io::BufRead;
use std::boxed;
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

type State = Box<HashMap<Object, usize>>;

// impl Hash for State {
// 	fn hash<H: Hasher>(&self, hasher: &mut H) {
// 		for (obj, floor) in self {
// 			obj.hash(hasher);
// 			floor.hash(hasher);
// 		}
//     }
// }

#[derive(PartialEq, Eq, Hash, Clone)]
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
        other.cost.cmp(&self.cost).then(Ordering::Equal)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for ReachedState {
    fn partial_cmp(&self, other: &ReachedState) -> Option<Ordering> {
    	match self.cmp(other) {
    		Ordering::Equal => None,
    		o => Some(o)
    	}
    }
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(start: State, goal: State) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: HashMap<State, usize> = HashMap::new();

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