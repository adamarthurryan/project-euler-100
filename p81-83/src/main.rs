use std::fmt;
use std::cmp::min;
use std::collections::{HashSet,HashMap};
#[test]
fn solves() {
    assert_eq!(solve_81(), 427337);
    assert_eq!(solve_82(), 260324);
    assert_eq!(solve_83(), 425185);
}

fn main() {
//    println!("Solution 81: {}", solve_81());      
//    println!("Solution 82: {}", solve_82());      
    println!("Solution 83: {}", solve_83());      
}

fn solve_81() -> usize {
    let matrix = Matrix::<usize>::parse_usize(include_str!("p081_matrix.txt"));
    let (w,h) = matrix.size();

    //create graph information for this matrix
    //require start and end nodes
    let start = Point::new(-1, 0);
    let end = Point::new(w as isize,(h-1) as isize);

    //neighbors function for edges 
    let neighbors = |p: Point| { 
        let mut points: Vec<Point> = Vec::new();
        //special handling for the end node
        if p==Point::new((w-1) as isize,(h-1) as isize) {
            points.push(end);
        }
        let pe = p.neighbor(Dir::E);
        if matrix.in_bounds(pe) {points.push(pe);}
        let ps = p.neighbor(Dir::S);
        if matrix.in_bounds(ps) {points.push(ps);}
        points
    };

    //costs for edges
    let costs = |_p0, p1| { 
        if p1==end { 0 }
        else { matrix.get(p1) } 
    };
  
    //find the optimal path
    lowest_cost_path(start, end, neighbors, costs).unwrap() 
}

fn solve_82() -> usize {
    let matrix = Matrix::<usize>::parse_usize(include_str!("p082_matrix.txt"));
    let (w,h) = matrix.size();

    //create graph information for this matrix
    //require start and end nodes
    let start = Point::new(-1, 0);
    let end = Point::new(w as isize,h as isize);

    //neighbors function for edges 
    let neighbors = |p: Point| { 
        let mut points: Vec<Point> = Vec::new();
        //the start node connects to all points in the first column
        if p==start {
            for i in 0..h {
                points.push(Point::new(0,i as isize));
            }
        }
        //the last column connects to the end node
        else if p.x == (w-1) as isize {
            points.push(end);
        }
        //the rest of the node connect up, right, and down
        else {         
            let dirs = [Dir::N, Dir::E, Dir::S];

            for dir in dirs {
                let pn = p.neighbor(dir);
                if matrix.in_bounds(pn) { points.push(pn); }
            }
        }

        points
    };

    //costs for edges
    let costs = |_p0, p1| { 
        if p1==end { 0 }
        else { matrix.get(p1) } 
    };
  
    //find the optimal path
    lowest_cost_path(start, end, neighbors, costs).unwrap() 
}

fn solve_83() -> usize {
    let matrix = Matrix::<usize>::parse_usize(include_str!("p083_matrix.txt"));
    let (w,h) = matrix.size();

    //create graph information for this matrix
    //require start and end nodes
    let start = Point::new(-1, 0);
    let end = Point::new(w as isize,(h-1) as isize);

    //neighbors function for edges 
    let neighbors = |p: Point| { 
        let mut points: Vec<Point> = Vec::new();
        //the start node connects to all points in the first column
        if p==start {
            points.push(Point::new(0,0));
        }
        //the last column connects to the end node
        else if p == Point::new((w-1) as isize, (h-1) as isize) {
            points.push(end);
        }
        //the rest of the node connect up, right, down, and left
        else {         
            let dirs = [Dir::N, Dir::E, Dir::S, Dir::W];

            for dir in dirs {
                let pn = p.neighbor(dir);
                if matrix.in_bounds(pn) { points.push(pn); }
            }
        }

        points
    };

    //costs for edges
    let costs = |_p0, p1| { 
        if p1==end { 0 }
        else { matrix.get(p1) } 
    };
  
    //find the optimal path
    lowest_cost_path(start, end, neighbors, costs).unwrap() 
}

fn lowest_cost_path<Node>(start: Node, end: Node, 
    neighbors: impl Fn(Node) -> Vec<Node>, 
    costs: impl Fn(Node, Node) -> usize 
) -> Option<usize> 
where Node:Eq+std::hash::Hash+Copy
{
    
    let mut node_cost: HashMap<Node, usize> = HashMap::new();
    node_cost.insert(start, 0);
    let mut search: HashSet<Node> = HashSet::new();
    search.insert(start);

    while !search.is_empty() {

        //pop a node from the search set
        let v = *search.iter().next().unwrap();
        search.remove(&v);

        //for each neighbor
        for u in neighbors(v) {
            //see if the neighbor can be more effieciently reached from this node's best path
            let cost = node_cost.get(&v).unwrap()+costs(v,u);
            if !node_cost.contains_key(&u) || cost < *node_cost.get(&u).unwrap() {
                //if so, update the cost and add the neighbor to the search queue
                node_cost.insert(u, cost);
                search.insert(u);
            }
        }
    }

    Some(*node_cost.get(&end).unwrap())
}


#[derive(Clone, Copy, PartialEq, Debug)]
enum Dir {
    N, E, S, W
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
struct Point {
    x: isize,
    y: isize
}

struct Matrix <T> {
    data: Vec<Vec<T>>
} 

impl Point {
    fn new(x: isize, y:isize) -> Point {
        Point{x,y}
    }

    fn neighbor(&self, dir:Dir) -> Point{
        match dir {
            Dir::N => Point::new(self.x, self.y-1),
            Dir::E => Point::new(self.x+1, self.y),
            Dir::S => Point::new(self.x, self.y+1),
            Dir::W => Point::new(self.x-1, self.y),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }

}

//should maybe implement this with pointers, skipping the copy trait
impl <T: Copy> Matrix <T> {
    fn new(w: usize, h:usize, initial:T) -> Matrix<T> {
        Matrix{ data: vec![vec![initial;w];h] }
    }

    fn in_bounds(&self, p: Point) -> bool {
        p.x>=0 && p.y>=0 && p.x<(self.data.len()) as isize && p.y<(self.data.len() as isize)
    }

    fn get(&self, p:Point) -> T {
        if !self.in_bounds(p) { panic!("tried to access an out-of-bounds cell") }
        
        self.data[p.y as usize][p.x as usize]
    }

    fn put(&mut self, p:Point, val:T) {
        if !self.in_bounds(p) {
            panic!("attempt to insert into invalid index");
        }

        self.data[p.y as usize][p.x as usize] = val;
    }

    fn size(&self) -> (usize, usize) {
        if self.data.is_empty() { (0, 0) }
        else { (self.data[1].len(), self.data.len()) }
    }


    fn parse_usize(source: &str) -> Matrix<usize> {
        Matrix{data: 
            source.split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line
                .split(',')
                .map(|cell| cell
                    .parse::<usize>()
                    .unwrap()
                ).collect()
            ).collect()
        }
    }
}

