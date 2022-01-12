mod djikstra;

use matrix::{Point, Dir, Matrix};
use crate::djikstra::lowest_cost_path;

#[test]
fn solves() {
    assert_eq!(solve_81(), 427337);
    assert_eq!(solve_82(), 260324);
    assert_eq!(solve_83(), 425185);
}

fn main() {
    println!("Solution 81: {}", solve_81());      
    println!("Solution 82: {}", solve_82());      
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
        //special handling for the start and end node
        if p==Point::new((w-1) as isize,(h-1) as isize) {
            points.push(end);
        }
        if p==start {
            points.push(Point::new(0,0))
        }
        let neighbors = matrix.neighbors(p);
        points.extend(neighbors.iter().filter(|(dir,_)| *dir==Dir::E || *dir==Dir::S).map(|(_,pn)| pn));
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
            let neighbors = matrix.neighbors(p);
            points.extend(neighbors.iter().filter(|(dir,_)| *dir==Dir::E || *dir==Dir::S || *dir==Dir::N).map(|(_,pn)| pn));
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
            let neighbors = matrix.neighbors(p);
            points.extend(neighbors.iter().map(|(_,pn)| pn));
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

