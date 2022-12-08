use std::str::FromStr;

#[derive(Clone, Eq, PartialEq)]
pub struct TreeGrid {
    grid: Vec<Vec<u8>>,
}

type Direction = (isize, isize);

impl TreeGrid {
    pub fn get(&self, row: usize, column: usize) -> Option<Tree> {
        let &height = self.grid.get(row)?.get(column)?;
        Some(Tree {
            height,
            row,
            column,
            grid: self,
        })
    }

    pub fn trees(&self) -> impl Iterator<Item = Tree> {
        self.grid.iter().enumerate().flat_map(move |(row, trees)| {
            trees.iter().enumerate().map(move |(column, &height)| Tree {
                row,
                column,
                height,
                grid: self,
            })
        })
    }
}

impl FromStr for TreeGrid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let grid = input
            .trim()
            .lines()
            .map(|line| line.chars().map(Tree::parse_height).collect())
            .collect();
        Ok(Self { grid })
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Tree<'a> {
    row: usize,
    column: usize,
    height: u8,
    grid: &'a TreeGrid,
}

impl<'a> Tree<'a> {
    fn parse_height(char: char) -> u8 {
        u8::try_from(char.to_digit(10).unwrap()).unwrap()
    }

    fn neighbour(&self, direction: Direction) -> Option<Tree<'a>> {
        let (row_delta, col_delta) = direction;
        let row = isize::try_from(self.row).unwrap() + row_delta;
        let column = isize::try_from(self.column).unwrap() + col_delta;
        if row < 0 || column < 0 {
            return None;
        }
        let row = usize::try_from(row).unwrap();
        let column = usize::try_from(column).unwrap();
        self.grid.get(row, column)
    }

    const fn trees_in_direction(self, direction: Direction) -> Treeterator<'a> {
        Treeterator {
            tree: Some(self),
            direction,
        }
    }

    pub fn visible(&self) -> bool {
        DIRECTIONS.iter().any(|&direction| {
            self.trees_in_direction(direction)
                .all(|tree| tree.height < self.height)
        })
    }

    pub fn scenic_score(&self) -> usize {
        DIRECTIONS
            .iter()
            .map(|&direction| {
                let mut distance = 0;
                for tree in self.trees_in_direction(direction) {
                    distance += 1;
                    if tree.height >= self.height {
                        break;
                    }
                }
                distance
            })
            .product()
    }
}

const DIRECTIONS: [Direction; 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

pub struct Treeterator<'a> {
    tree: Option<Tree<'a>>,
    direction: Direction,
}

impl<'a> Iterator for Treeterator<'a> {
    type Item = Tree<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.tree = self.tree?.neighbour(self.direction);
        self.tree
    }
}
