
use std::cmp::Ordering;
use std::collections::{HashMap,BinaryHeap};

//some helper methods for Djikstra's algorithm
#[derive(Copy, Clone, Eq, PartialEq)]
struct SearchItem <Node> 
where Node:Eq
{
    node: Node,
    cost: usize
}

impl <Node> Ord for SearchItem <Node>
where Node:Eq {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}
impl <Node> PartialOrd for SearchItem <Node> 
where Node:Eq
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//Djikstra's algorithm
pub fn lowest_cost_path<Node>(start: Node, end: Node, 
    neighbors: impl Fn(Node) -> Vec<Node>, 
    costs: impl Fn(Node, Node) -> usize 
) -> Option<usize> 
where Node:Eq+std::hash::Hash+Copy
{    
    let mut node_cost: HashMap<Node, usize> = HashMap::new();
    node_cost.insert(start, 0);

    let mut heap = BinaryHeap::new();
    heap.push(SearchItem{node:start, cost:0});

    while let Some(SearchItem { cost, node:v }) = heap.pop() {
        if v == end {
            return Some(cost);
        }
        //for each neighbor
        for u in neighbors(v) {
            //see if the neighbor can be more effieciently reached from this node's best path
            let new_cost = cost+costs(v,u);
            if !node_cost.contains_key(&u) || new_cost < *node_cost.get(&u).unwrap() {
                //if so, update the cost and add the neighbor to the search queue
                node_cost.insert(u, cost);
                heap.push(SearchItem{node:u, cost:new_cost});
            }
        }
    }

    Some(*node_cost.get(&end).unwrap())
}
