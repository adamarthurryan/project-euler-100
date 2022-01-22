/*The primes 3, 7, 109, and 673, are quite remarkable. By taking any two primes and 
concatenating them in any order the result will always be prime. For example, 
taking 7 and 109, both 7109 and 1097 are prime. The sum of these four primes, 792, 
represents the lowest sum for a set of four primes with this property.

Find the lowest sum for a set of five primes for which any two primes concatenate 
to produce another prime.*/

use std::collections::{HashSet};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concatenate_works() {
        assert_eq!(concatenate(55,68), 5568);
        assert_eq!(concatenate(3, 123091), 3123091);
        assert_eq!(concatenate(213,8), 2138);
    }

    #[test]
    fn solves_problem() {
        assert_eq!(solve(), Some(26033));
    }
}



fn main() {
    let solution = solve();
    println!("Solution: {}", solution.expect("No solution found."));

}

fn solve () -> Option<u64> {
    let sieve = primal::Sieve::new(100_000_000);
    let primes: Vec<u64> = sieve.primes_from(3).take_while(|&p| p<10_000).map(|p| p as u64).collect();

    //find all pairs of primes
    let pairs = Pairs::new(&primes);


    //filter for both composites are prime
    let pairs: Vec<_> = pairs.into_iter().filter(|(&a,&b)| sieve.is_prime(concatenate(a,b) as usize) && sieve.is_prime(concatenate(b,a) as usize)).collect();
    

    //now we are in graph theory
    //build a graph representation with a hashmap of vertex neighbors
    let mut neighbors: HashSet<(u64,u64)> = HashSet::new();
    let mut vertices: HashSet<u64> = HashSet::new();


    for (a,b) in pairs {
        //add the edge to the neighbours map both ways
        for (u,v) in [(a,b),(b,a)] {
            neighbors.insert((*u,*v));
            vertices.insert(*u);
        }
    }

    let mut vertices: Vec<u64> = vertices.into_iter().collect();
    vertices.sort_unstable();

//    //drop any vertices that don't have at least 5 neighbors
//    vertices = vertices.into_iter().filter(|v| neighbors.get(&v).unwrap().len()>=5).collect();
    
    //println!("{:?}", pairs);
//    println!("{:?}", neighbors);
//    println!("{:?}", vertices);


    //todo: "Refactor back to bron_kerbosch algorithm?"

    //find a k-clique
    let clique = find_k_clique(5, &vertices, &neighbors);
    match clique {
        Some(clique) => {
            println!("5-clique found: {:?}", clique);
            Some(clique.into_iter().sum())  
        }
        _ => {
            println!("no 5-clique found");
            None
        }
    }

}



//this could maybe run better without hashsets
//eg sorted iterators or something?
fn find_k_clique(k: usize, vertices: &[u64], neighbors: &HashSet<(u64, u64)>) -> Option<Vec<u64>> {
    k_clique_inner(k, 0, &Vec::new(), vertices, neighbors)
}

//another try for a k-clique algorithm
//maybe more efficient
//this should start from pairs, not just individual items, right?
fn k_clique_inner(k:usize, pivot:usize, clique: &[u64], vertices: &[u64], neighbors: &HashSet<(u64, u64)> ) -> Option<Vec<u64>> {
    let mut pivot = pivot;

    for v in &vertices[pivot..] {
        pivot += 1;
        //if this vertex is in the clique
        if clique.iter().all(|u| neighbors.contains(&(*u,*v))) {
            let mut new_clique = clique.to_vec();
            new_clique.push(*v);

            if new_clique.len() == k {
                return Some(new_clique.to_vec());
            }
            //recurse from this point
            if let Some(result) = k_clique_inner(k, pivot, &new_clique, vertices, neighbors) {
                return Some(result);
            }
        }
    }

    None
}

/*
fn bron_kerbosch(r: &HashSet<u64>, p: &mut HashSet<u64>, x: &mut HashSet<u64>, neighbors: &mut HashMap<u64, Vec<u64>>) -> Vec<Vec<u64>> {
    // from Wikipedia:
    // algorithm BronKerbosch1(R, P, X) is
    // if P and X are both empty then
    // report R as a maximal clique
    // for each vertex v in P do
    // BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
    // P := P \ {v}
    // X := X ⋃ {v}

    //if P and X are both empty then
    //    report R as a maximal clique 
    if p.len() == 0 && x.len() == 0 {
        return vec![r.into_iter().map(|&x| x).collect()];
    }
    //for each vertex v in P do

    let mut cliques = Vec::new();
    let mut pp: HashSet<_> = p.iter().map(|&x| x).collect();
    for v in p.iter() {
        //BronKerbosch1(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        let mut v_set = HashSet::new();
        v_set.insert(*v);
        let mut v_neighbors: HashSet<_> = neighbors.get(&v).unwrap().iter().map(|&x| x).collect();
        cliques.append(&mut bron_kerbosch(
            &mut r.union(&v_set).map(|&x| x).collect(), 
            &mut pp.intersection(&v_neighbors).map(|&x| x).collect(), 
            &mut x.intersection(&v_neighbors).map(|&x| x).collect(), 
            neighbors));

        //P := P \ {v}
        //X := X ⋃ {v}
        x.insert(*v);
        pp.remove(v);
 
    }
    return cliques;

    //unimplemented!();
}
*/

//concatenate the digits of b after the digits of a
fn concatenate(a:u64, b:u64) -> u64 {
    let exp = (b as f64).log10().ceil() as u32;
    a*10u64.pow(exp)+b
}

struct Pairs <'a, T> {
    next: (usize,usize),
    source: &'a [T],
}

impl <'a, T> Pairs <'a, T> {
    fn new(source:&[T]) -> Pairs <T> {
        Pairs{next:(0,1), source}
    }
}

impl <'a, T> Iterator for Pairs <'a, T> {
    type Item = (&'a T, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.next.0 >= self.source.len() || self.next.1 >= self.source.len() {
            return None;
        } 

        let item = (&self.source[self.next.0], &self.source[self.next.1]);
        self.next.1 += 1;
        if self.next.1 >= self.source.len() {
            self.next.0 += 1;
            self.next.1 = self.next.0 + 1;
        }

        Some(item)
    }
}