use std::fmt;

#[cfg(test)]
mod tests {
}




#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Dir {
    N, E, S, W
}

#[derive(Clone, Copy, PartialEq, Debug, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize
}

pub struct Matrix <T> {
    data: Vec<Vec<T>>
} 

impl Point {
    pub fn new(x: isize, y:isize) -> Point {
        Point{x,y}
    }

}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_fmt(format_args!("({},{})", self.x, self.y))
    }

}

//should maybe implement this with pointers, skipping the copy trait
impl <T: Copy> Matrix <T> {
    pub fn new(w: usize, h:usize, initial:T) -> Matrix<T> {
        Matrix{ data: vec![vec![initial;w];h] }
    }

    pub fn neighbor(&self, p: Point, dir: Dir) -> Option<Point> {
        let new_point = match dir {
            Dir::N => Point::new(p.x, p.y-1),
            Dir::E => Point::new(p.x+1, p.y),
            Dir::S => Point::new(p.x, p.y+1),
            Dir::W => Point::new(p.x-1, p.y),
        };

        if self.in_bounds(new_point) {
            Some(new_point)
        }
        else {
            None
        }
    }

    pub fn neighbors(&self, p: Point) -> Vec<(Dir,Point)> {
        let mut points = Vec::new();
        for dir in [Dir::N, Dir::E, Dir::S, Dir::W] {
            if let Some(point) = self.neighbor(p, dir) {
                points.push((dir, point));
            }
        }
        points
    }

    pub fn in_bounds(&self, p: Point) -> bool {
        p.x>=0 && p.y>=0 && p.x<(self.data.len()) as isize && p.y<(self.data.len() as isize)
    }

    pub fn get(&self, p:Point) -> T {
        if !self.in_bounds(p) { panic!("tried to access an out-of-bounds cell") }
        
        self.data[p.y as usize][p.x as usize]
    }

    pub fn put(&mut self, p:Point, val:T) {
        if !self.in_bounds(p) { panic!("attempt to insert into an out-of-bounds cell"); }

        self.data[p.y as usize][p.x as usize] = val;
    }

    pub fn size(&self) -> (usize, usize) {
        if self.data.is_empty() { (0, 0) }
        else { (self.data[1].len(), self.data.len()) }
    }

    pub fn parse_usize(source: &str) -> Matrix<usize> {
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

