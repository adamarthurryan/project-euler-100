#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_problem() {
        assert_eq!(solve(), Some(28684));
    }
}

fn main() {
    let solution = solve();
    println!("Solution: {}", solution.expect("No solution found."));
}

fn solve() -> Option<usize> {
    
    
    /* Polygonal Numbers
    Triangle	 	P3,n=n(n+1)/2	 	1, 3, 6, 10, 15, ...
    Square	 	    P4,n=n2	 	1, 4, 9, 16, 25, ...
    Pentagonal	 	P5,n=n(3n−1)/2	 	1, 5, 12, 22, 35, ...
    Hexagonal	 	P6,n=n(2n−1)	 	1, 6, 15, 28, 45, ...
    Heptagonal	 	P7,n=n(5n−3)/2	 	1, 7, 18, 34, 55, ...
    Octagonal	 	P8,n=n(3n−2)	 	1, 8, 21, 40, 65, ...
    */

    let triangle =      (1..).map(|n| n*(n+1)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();
    let squares =       (1..).map(|n| n*n).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();
    let pentagonal =    (1..).map(|n| n*(3*n-1)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();
    let hexagonal =     (1..).map(|n| n*(2*n-1)).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();
    let heptagonal =    (1..).map(|n| n*(5*n-3)/2).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();
    let octagonal =     (1..).map(|n| n*(3*n-2)).skip_while(|&x| x<1000).take_while(|&x| x<10000).collect::<Vec<usize>>();

    let nums = [octagonal, triangle, squares, pentagonal, hexagonal, heptagonal];
    

    //one way to look at this is thinking of the numbers as a k-partite graph 
    //then we are trying to find a cycle that visits each partition once
    let result = find_cycle(&nums);

    //if we found a cycle, process it and return
    match result {
        Some(cycle) => {
            println!("cycle: {:?}", cycle);
            return Some(cycle.into_iter().map(|(_part, num)| num).sum());
        }
        None => {
            return None;
        }
    }
}

//true if there is an edge from a->b
//ie if the last two digits of a are equal to the first two digits of b
fn is_edge(a:usize, b:usize) -> bool {
    b/100 == a%100
}

//find a cycle that touches each partition fo the graph at least once
//edges are defined by the is_edge function in this namespace
fn find_cycle(nums: &[Vec<usize>]) -> Option<Vec<(usize, usize)>> {
    find_cycle_inner(nums, 0, 0, &Vec::new(), &vec![false; 6])
}

fn find_cycle_inner(nums:  &[Vec<usize>], part: usize, step: usize, path: &[(usize, usize)], visited: &[bool]) -> Option<Vec<(usize, usize)>> {
    for &a in &nums[part] {
        //see if this a can extend the cycle
        if step == 0 || is_edge(path[step-1].1, a) {
            //update the path and visited parameters
            let mut path = path.to_vec(); 
            path.push((part, a));

            let mut visited = visited.to_vec();
            visited[part] = true;

            //if this is the last step and the path is a cycle, return success
            if step+1 == 6 && is_edge(path[5].1, path[0].1) {
                return Some(path)
            }

            //otherwise recurse on each unvisited partition        
            let next_parts = (0..=5).filter(|&part| !visited[part]);
            for next_part in next_parts {
                match find_cycle_inner(nums, next_part, step+1, &path, &visited) {
                    Some(cycle) => { return Some(cycle); }
                    None => {}
                }
            }
        }
    }
    return None;
}
