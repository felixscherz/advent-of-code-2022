#[derive(Copy, Clone)]
pub struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn to_tuple(&self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Move {
    direction: Direction,
    amount: i64,
}

impl Move {
    fn opposite(&self) -> Self {
        match self.direction {
            Direction::Up => Self {
                direction: Direction::Down,
                amount: self.amount,
            },
            Direction::Right => Self {
                direction: Direction::Left,
                amount: self.amount,
            },
            Direction::Down => Self {
                direction: Direction::Up,
                amount: self.amount,
            },
            Direction::Left => Self {
                direction: Direction::Right,
                amount: self.amount,
            },
        }
    }

    pub fn from_line(l: &String) -> Self {
        let direction = match l.split_whitespace().nth(0).unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            a => panic!("found unexpected move {}", a),
        };
        let amount = l
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<i64>()
            .unwrap();
        Self { direction, amount }
    }

    pub fn break_down(self) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for _ in 0..self.amount {
            moves.push(Self{
                direction: self.direction,
                amount: 1,
            })
        };
        moves
    }
}

impl Point {
    fn moved(&self, mv: &Move) -> Self {
        let new_point: Point = match mv {
            Move {
                direction: Direction::Up,
                amount,
            } => Point {
                x: self.x,
                y: self.y + amount,
            },
            Move {
                direction: Direction::Right,
                amount,
            } => Point {
                x: self.x + amount,
                y: self.y,
            },
            Move {
                direction: Direction::Down,
                amount,
            } => Point {
                x: self.x,
                y: self.y - amount,
            },
            Move {
                direction: Direction::Left,
                amount,
            } => Point {
                x: self.x - amount,
                y: self.y,
            },
        };
        new_point
    }
}

pub struct Rope {
    pub head: Point,
    pub tail: Point,
}

impl Rope {
    pub fn move_rope(&self, mv: &Move) -> Self {
        let new_head_position: Point = self.head.moved(mv);
        let new_tail_position: Point = match self.tail_distance(&new_head_position) {
            0 | 1 | 2 => self.tail.clone(), // tail on top, next to or diagonal
            _ => new_head_position.moved(&mv.opposite()),
        };
        Self {
            head: new_head_position,
            tail: new_tail_position,
        }
    }

    fn tail_distance(&self, head: &Point) -> i64 {
        let dx = (head.x - self.tail.x) * (head.x - self.tail.x);
        let dy = (head.y - self.tail.y) * (head.y - self.tail.y);
        dx + dy
    }
}
