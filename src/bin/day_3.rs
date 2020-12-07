use std::{fmt::Display, fs::read_to_string, ops::Index, str::FromStr};

#[derive(Debug)]
enum Location {
    Open,
    Tree,
}

#[derive(Debug)]
struct Map {
    data: Vec<Location>,
    width: usize,
    height: usize,
}

impl Map {
    fn count_trees_on_slope(&self, right: usize, down: usize) -> usize {
        let (mut i, mut j) = (0, 0);
        let mut trees = 0;

        while j < self.height {
            match self[(i, j)] {
                Location::Open => {}
                Location::Tree => trees += 1,
            }
            i += right;
            j += down;
        }
        trees
    }
}

impl FromStr for Map {
    type Err = ();

    // Assumes all rows are the same width
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = vec![];

        let mut height = 0;
        let mut width = 0;

        for line in s.lines() {
            width = 0;
            height += 1;
            for c in line.chars() {
                width += 1;
                let location = match c {
                    '.' => Location::Open,
                    '#' => Location::Tree,
                    _ => return Err(()),
                };
                data.push(location);
            }
        }
        Ok(Map {
            data,
            width,
            height,
        })
    }
}

impl Index<(usize, usize)> for Map {
    type Output = Location;
    fn index(&self, idx: (usize, usize)) -> &Self::Output {
        let (i, j) = idx;
        let i = i % self.width;
        self.data
            .chunks(self.width)
            .nth(j)
            .unwrap()
            .iter()
            .nth(i)
            .unwrap()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "width: {}, height: {}", self.width, self.height)?;
        for line in self.data.chunks(self.width) {
            for location in line {
                match location {
                    Location::Open => write!(f, ".")?,
                    Location::Tree => write!(f, "#")?,
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

fn part_1(map: &Map) -> usize {
    map.count_trees_on_slope(3, 1)
}

fn part_2(map: &Map) -> usize {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut product = 1;
    for (right, down) in slopes {
        product *= map.count_trees_on_slope(right, down)
    }
    product
}

fn main() {
    let input = read_to_string("inputs/day_3").expect("Error reading input");
    let map = Map::from_str(&input).expect("Error parsing map input");
    println!("Part 1: {}", part_1(&map));
    println!("Part 2: {}", part_2(&map));
}
