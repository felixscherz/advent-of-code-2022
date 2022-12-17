pub enum Direction {
    North,
    South,
    East,
    West,
}

pub struct Grid {
    pub grid: Vec<Vec<u32>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn from_input(input: &Vec<String>) -> Self {
        let mut grid: Vec<Vec<u32>> = Vec::new();
        for line in input.iter() {
            grid.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
        let width: usize = grid.last().unwrap().len();
        let height: usize = grid.len();
        Self {
            grid,
            width,
            height,
        }
    }

    pub fn tree_visible(&self, i: usize, j: usize, from: Direction) -> bool {
        let trees_between: Vec<&u32> = match from {
            Direction::North => self.trees_from_north(i, j),
            Direction::East => self.trees_from_east(i, j),
            Direction::South => self.trees_from_south(i, j),
            Direction::West => self.trees_from_west(i, j),
        };
        let height = self.grid.get(i).unwrap().get(j).unwrap();
        for tree in trees_between {
            if tree >= height {
                return false;
            }
        }
        true
    }

    pub fn trees_visible(&self, i: usize, j: usize, to: Direction) -> usize {
        let trees_to_border: Vec<&u32> = match to {
            Direction::North => self.trees_from_north(i, j),
            Direction::East => self.trees_from_east(i, j),
            Direction::South => self.trees_from_south(i, j),
            Direction::West => self.trees_from_west(i, j),
        }.into_iter().rev().collect();
        let height = self.grid.get(i).unwrap().get(j).unwrap();
        let mut trees: Vec<&u32> = Vec::new();
        for tree in trees_to_border {
            trees.push(tree);
            if tree >= height {
                break;
            }
        }
        trees.iter().count()

    }

    fn trees_from_north(&self, i: usize, j: usize) -> Vec<&u32> {
        let mut treeline: Vec<&u32> = Vec::new();
        if i == 0 {
            return treeline;
        }
        for l in 0..i {
            treeline.push(self.grid.get(l).unwrap().get(j).unwrap());
        }
        treeline
    }

    fn trees_from_south(&self, i: usize, j: usize) -> Vec<&u32> {
        let mut treeline: Vec<&u32> = Vec::new();
        if i == self.height-1 {
            return treeline;
        }
        for l in (i+1..self.height).rev() {
            treeline.push(self.grid.get(l).unwrap().get(j).unwrap());
        }
        treeline
    }

    fn trees_from_east(&self, i: usize, j: usize) -> Vec<&u32> {
        let mut treeline: Vec<&u32> = Vec::new();
        if j == self.width-1 {
            return treeline;
        }
        for k in (j+1..self.width).rev() {
            treeline.push(self.grid.get(i).unwrap().get(k).unwrap());
        }
        treeline
    }

    fn trees_from_west(&self, i: usize, j: usize) -> Vec<&u32> {
        let mut treeline: Vec<&u32> = Vec::new();
        if j == 0 {
            return treeline;
        }
        for k in 0..j {
            treeline.push(self.grid.get(i).unwrap().get(k).unwrap());
        }
        treeline
    }
}
