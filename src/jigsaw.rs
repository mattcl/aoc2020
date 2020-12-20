use crate::error::{AocError, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Side {
    Top,
    Bot,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Offset(usize, usize);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Dimension(usize, usize);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Edge(u128);

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Tile {
    pub id: usize,
    data: Vec<Vec<char>>,
    dimensions: Dimension,
    top: Edge,
    bot: Edge,
    left: Edge,
    right: Edge,
}

impl Tile {
    pub fn new(id: usize, data: &Vec<Vec<char>>) -> Result<Self> {
        let edges = Self::make_edges(data)?;
        Ok(Tile {
            id: id,
            data: data.clone(),
            dimensions: Dimension(data.len(), data[0].len()),
            top: edges.0,
            bot: edges.1,
            left: edges.2,
            right: edges.3,
        })
    }

    pub fn get_edge(&self, side: &Side) -> Edge {
        match side {
            Side::Top => self.top.clone(),
            Side::Bot => self.bot.clone(),
            Side::Left => self.left.clone(),
            Side::Right => self.right.clone(),
        }
    }

    fn make_edges(data: &Vec<Vec<char>>) -> Result<(Edge, Edge, Edge, Edge)> {
        let top = Edge(u128::from_str_radix(
            &data[0]
                .iter()
                .collect::<String>()
                .replace('.', "0")
                .replace('#', "1"),
            2,
        )?);

        let bot = Edge(u128::from_str_radix(
            &data[data.len() - 1]
                .iter()
                .collect::<String>()
                .replace('.', "0")
                .replace('#', "1"),
            2,
        )?);

        let left = Edge(u128::from_str_radix(
            &(0..data.len())
                .map(|row| data[row][0])
                .collect::<String>()
                .replace('.', "0")
                .replace('#', "1"),
            2,
        )?);

        let right = Edge(u128::from_str_radix(
            &(0..data.len())
                .map(|row| data[row][data[0].len() - 1])
                .collect::<String>()
                .replace('.', "0")
                .replace('#', "1"),
            2,
        )?);

        Ok((top, bot, left, right))
    }

    pub fn from_input(input: &[String]) -> Result<Self> {
        let mut parts = input.iter();
        if let Some(title) = parts.next() {
            if let Some(id) = title
                .strip_suffix(":")
                .unwrap_or("invalidinput")
                .split(' ')
                .skip(1)
                .next()
            {
                let data = parts
                    .map(|line| line.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>();

                if !data.is_empty() && data.iter().map(|row| row.len()).unique().count() == 1 {
                    return Ok(Self::new(id.parse::<usize>()?, &data)?);
                }
            }
        }

        Err(AocError::InvalidInput(format!(
            "Could not construct tile from input: {:#?}",
            input
        )))
    }

    pub fn flip_horizontal(&self) -> Self {
        let mut new_data = self.data.clone();
        new_data.iter_mut().for_each(|row| row.reverse());
        Self::new(self.id, &new_data).unwrap()
    }

    pub fn flip_vertical(&self) -> Self {
        let mut new_data = self.data.clone();
        new_data.reverse();
        Self::new(self.id, &new_data).unwrap()
    }

    pub fn rotate_right(&self) -> Self {
        let mut new_data = vec![Vec::with_capacity(self.dimensions.0); self.dimensions.1];

        for col in 0..self.dimensions.1 {
            for row in 0..self.dimensions.0 {
                if let Some(data) = new_data.get_mut(col) {
                    data.push(self.data[self.dimensions.0 - 1 - row][col]);
                }
            }
        }

        Self::new(self.id, &new_data).unwrap()
    }

    pub fn rotate_left(&self) -> Self {
        let mut new_data = vec![Vec::with_capacity(self.dimensions.0); self.dimensions.1];

        for col in 0..self.dimensions.1 {
            for row in 0..self.dimensions.0 {
                if let Some(data) = new_data.get_mut(col) {
                    data.push(self.data[row][self.dimensions.1 - 1 - col]);
                }
            }
        }

        Self::new(self.id, &new_data).unwrap()
    }

    pub fn find_shape(&mut self, shape: &Vec<Vec<char>>) -> usize {
        let mut offsets: Vec<Offset> = Vec::new();
        let mut count = 0;

        for (r, row) in shape.iter().enumerate() {
            for (c, col) in row.iter().enumerate() {
                if *col == '#' {
                    offsets.push(Offset(r, c));
                }
            }
        }

        for r in 0..self.dimensions.0 {
            for c in 0..self.dimensions.1 {
                if self.matches_shape(r, c, &offsets) {
                    count += 1;
                    for offset in &offsets {
                        self.data[r + offset.0][c + offset.1] = 'O';
                    }
                }
            }
        }

        count
    }

    pub fn matches_shape(&self, row: usize, col: usize, offsets: &Vec<Offset>) -> bool {
        for offset in offsets {
            if let Some(row) = self.data.get(offset.0 + row) {
                if let Some(col) = row.get(offset.1 + col) {
                    if *col != '#' {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }

    pub fn count_char(&self, ch: char) -> usize {
        self.data
            .iter()
            .map(|row| row.iter().filter(|c| **c == ch).count())
            .sum()
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = self
            .data
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "Tile {}:\n{}", self.id, data)
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Rotation {
    Normal,
    Ninety,
    OneEighty,
    TwoSeventy,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Vertical {
    Normal,
    Flipped,
}

#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub enum Horizontal {
    Normal,
    Flipped,
}

// (tile_id, variant_id)
#[derive(Debug, Eq, PartialEq, Clone, Hash)]
pub struct Variant(pub usize, pub Rotation, pub Horizontal, pub Vertical);

#[derive(Debug, PartialEq, Clone)]
pub struct Grid {
    tiles: HashMap<usize, Tile>,
    pub variant_map: HashMap<Variant, Tile>,
    pub edge_map: HashMap<Side, HashMap<Edge, Vec<Variant>>>,
    pub arrangement: Vec<Vec<Option<Tile>>>,
    dimensions: Dimension,
}

impl Grid {
    pub fn new(tiles: &HashMap<usize, Tile>) -> Self {
        let len = tiles.len() as f64;
        let edge_len = len.sqrt() as usize;

        let mut g = Grid {
            tiles: tiles.clone(),
            variant_map: HashMap::new(),
            edge_map: HashMap::new(),
            dimensions: Dimension(edge_len, edge_len),
            arrangement: vec![vec![None; edge_len]; edge_len],
        };

        g.make_variant_map();
        g.make_edge_map();

        g
    }
    pub fn from_input(input: &[String]) -> Result<Self> {
        let mut tiles = HashMap::new();
        let _ = input
            .split(|line| line.is_empty())
            .map(|lines| Tile::from_input(lines))
            .collect::<Result<Vec<Tile>>>()?
            .into_iter()
            .map(|tile| tiles.insert(tile.id, tile))
            .collect::<Vec<Option<Tile>>>();

        Ok(Self::new(&tiles))
    }

    fn make_variant_map(&mut self) {
        let rotations = vec![
            Rotation::Normal,
            Rotation::Ninety,
            Rotation::OneEighty,
            Rotation::TwoSeventy,
        ];

        let h_flips = vec![Horizontal::Normal, Horizontal::Flipped];

        let v_flips = vec![Vertical::Normal, Vertical::Flipped];

        for (id, tile) in &self.tiles {
            let mut rot_cache = tile.clone();
            for rotation in &rotations {
                if rotation != &Rotation::Normal {
                    rot_cache = rot_cache.rotate_right();
                }

                for h in &h_flips {
                    if (h == &Horizontal::Flipped && rotation == &Rotation::OneEighty)
                        || (h == &Horizontal::Flipped && rotation == &Rotation::TwoSeventy)
                    {
                        continue;
                    }

                    let horizontally_flipped = match h {
                        Horizontal::Normal => rot_cache.clone(),
                        Horizontal::Flipped => rot_cache.flip_horizontal(),
                    };

                    for v in &v_flips {
                        if (v == &Vertical::Flipped && h == &Horizontal::Flipped)
                            || (v == &Vertical::Flipped && rotation == &Rotation::TwoSeventy)
                            || (v == &Vertical::Flipped && rotation == &Rotation::OneEighty)
                        {
                            continue;
                        }

                        let vertically_flipped = match v {
                            Vertical::Normal => horizontally_flipped.clone(),
                            Vertical::Flipped => horizontally_flipped.flip_vertical(),
                        };

                        self.variant_map.insert(
                            Variant(*id, rotation.clone(), h.clone(), v.clone()),
                            vertically_flipped,
                        );
                    }
                }
            }
        }
    }

    pub fn make_edge_map(&mut self) {
        let sides = vec![Side::Top, Side::Left];
        for (v, t) in &self.variant_map {
            for side in sides.iter() {
                let edge = t.get_edge(side);
                self.edge_map
                    .entry(side.clone())
                    .or_insert(HashMap::new())
                    .entry(edge)
                    .or_insert(Vec::new())
                    .push(v.clone());
            }
        }
    }

    pub fn dimensions(&self) -> &Dimension {
        &self.dimensions
    }

    pub fn num_tiles(&self) -> usize {
        self.tiles.len()
    }

    pub fn get_corner_product(&self) -> Result<usize> {
        let mut product = 1;

        if let Some(ref t) = self.arrangement[0][0] {
            product *= t.id;
        } else {
            return Err(AocError::ArrangementNotFound);
        }

        if let Some(ref t) = self.arrangement[0][self.dimensions.1 - 1] {
            product *= t.id;
        } else {
            return Err(AocError::ArrangementNotFound);
        }

        if let Some(ref t) = self.arrangement[self.dimensions.0 - 1][0] {
            product *= t.id;
        } else {
            return Err(AocError::ArrangementNotFound);
        }

        if let Some(ref t) = self.arrangement[self.dimensions.0 - 1][self.dimensions.1 - 1] {
            product *= t.id;
        } else {
            return Err(AocError::ArrangementNotFound);
        }

        Ok(product)
    }

    pub fn arrange(&mut self) -> bool {
        let mut available = HashMap::new();
        let mut arrangement = vec![vec![None; self.dimensions.1]; self.dimensions.0];

        for v in self.variant_map.keys() {
            available.entry(v.0).or_insert(Vec::new()).push(v.clone());
        }

        for (id, variants) in available.iter() {
            let mut new_available = available.clone();
            new_available.remove(id);

            for variant in variants {
                if let Some(tile) = self.variant_map.get(variant) {
                    arrangement[0][0] = Some(tile.clone());

                    if self.recur(
                        0,
                        1,
                        Some(tile.get_edge(&Side::Right)),
                        &new_available,
                        &mut arrangement,
                    ) {
                        self.arrangement = arrangement;
                        return true;
                    }

                    arrangement[0][0] = None;
                }
            }
        }
        false
    }

    fn recur(
        &self,
        row: usize,
        col: usize,
        right: Option<Edge>,
        available: &HashMap<usize, Vec<Variant>>,
        arrangement: &mut Vec<Vec<Option<Tile>>>,
    ) -> bool {
        if row == self.dimensions.0 {
            return true;
        }

        let bot = if row > 0 {
            match arrangement[row - 1].get(col).unwrap() {
                Some(t) => Some(t.get_edge(&Side::Bot)),
                None => None,
            }
        } else {
            None
        };

        let variants = self.get_variants(&right, &bot, available);
        for variant in variants {
            if let Some(tile) = self.variant_map.get(variant) {
                if let Some(ref bot) = bot {
                    if *bot != tile.get_edge(&Side::Top) {
                        continue;
                    }
                }

                let mut new_available = available.clone();
                new_available.remove(&variant.0);

                arrangement[row][col] = Some(tile.clone());

                let mut right = Some(tile.get_edge(&Side::Right));
                let mut next_row = row;
                let mut next_col = col + 1;

                if next_col >= self.dimensions.1 {
                    next_row += 1;
                    next_col = 0;
                    right = None;
                }

                if self.recur(next_row, next_col, right, &new_available, arrangement) {
                    return true;
                }

                arrangement[row][col] = None;
            }
        }

        false
    }

    fn get_variants(
        &self,
        right: &Option<Edge>,
        bot: &Option<Edge>,
        available: &HashMap<usize, Vec<Variant>>,
    ) -> Vec<&Variant> {
        if let Some(right) = right {
            if let Some(candidates) = self.edge_map.get(&Side::Left) {
                if let Some(variants) = candidates.get(&right) {
                    return variants
                        .iter()
                        .filter(|v| available.contains_key(&v.0))
                        .collect();
                }
            }
        } else if let Some(bot) = bot {
            if let Some(candidates) = self.edge_map.get(&Side::Top) {
                if let Some(variants) = candidates.get(&bot) {
                    return variants
                        .iter()
                        .filter(|v| available.contains_key(&v.0))
                        .collect();
                }
            }
        }
        unreachable!();
    }

    pub fn make_complete_tile(&self) -> Option<Tile> {
        if let Some(ref first_tile) = self.arrangement[0][0] {
            let rows_per_tile = first_tile.dimensions.0 - 2;
            let cols_per_tile = first_tile.dimensions.1 - 2;
            let mut data: Vec<Vec<char>> = vec![Vec::new(); rows_per_tile * self.dimensions.0];

            for (index, row) in self.arrangement.iter().enumerate() {
                // 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 -> 10
                for tile_row in 0..rows_per_tile {
                    for col in row {
                        data.get_mut(index * rows_per_tile + tile_row)?.extend(
                            col.clone()?.data[tile_row + 1]
                                .iter()
                                .skip(1)
                                .take(cols_per_tile),
                        );
                    }
                }
            }

            if let Ok(tile) = Tile::new(111111, &data) {
                return Some(tile);
            }
        }

        None
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self
            .arrangement
            .iter()
            .map(|row| {
                row.iter()
                    .map(|col| match col {
                        Some(t) => format!("{}", t.id),
                        None => "XXXX".to_string(),
                    })
                    .collect::<Vec<String>>()
                    .join("    ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_input;

    mod tile {
        use super::*;

        #[test]
        fn from_input() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###
            ",
            );

            let t = Tile::from_input(&input).unwrap();

            assert_eq!(format!("{}", t), input.join("\n"));
            assert_eq!(t.top, Edge(210));
            assert_eq!(t.bot, Edge(231));
            assert_eq!(t.left, Edge(498));
            assert_eq!(t.right, Edge(89));
        }

        #[test]
        fn flip_vertical() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###
            ",
            );

            let expected_input = test_input(
                "
                Tile 2311:
                ..###..###
                ###...#.#.
                ..#....#..
                .#.#.#..##
                ##...#.###
                ##.##.###.
                ####.#...#
                #...##..#.
                ##..#.....
                ..##.#..#.
            ",
            );

            let expected = Tile::from_input(&expected_input).unwrap();
            let t = Tile::from_input(&input).unwrap();

            assert_eq!(t.flip_vertical(), expected);
        }

        #[test]
        fn flip_horizontal() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###
            ",
            );

            let expected_input = test_input(
                "
                Tile 2311:
                .#..#.##..
                .....#..##
                .#..##...#
                #...#.####
                .###.##.##
                ###.#...##
                ##..#.#.#.
                ..#....#..
                .#.#...###
                ###..###..
            ",
            );

            let expected = Tile::from_input(&expected_input).unwrap();
            let t = Tile::from_input(&input).unwrap();

            assert_eq!(t.flip_horizontal(), expected);
        }

        #[test]
        fn rotate_right() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###
            ",
            );

            let expected_input = test_input(
                "
                Tile 2311:
                .#..#####.
                .#.####.#.
                ###...#..#
                #..#.##..#
                #....#.##.
                ...##.##.#
                .#...#....
                #.#.##....
                ##.###.#.#
                #..##.#...
            ",
            );

            let expected = Tile::from_input(&expected_input).unwrap();

            let t = Tile::from_input(&input).unwrap();
            assert_eq!(t.rotate_right(), expected);
        }

        #[test]
        fn rotate_left() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###
            ",
            );

            let expected_input = test_input(
                "
                Tile 2311:
                ...#.##..#
                #.#.###.##
                ....##.#.#
                ....#...#.
                #.##.##...
                .##.#....#
                #..##.#..#
                #..#...###
                .#.####.#.
                .#####..#.
            ",
            );

            let expected = Tile::from_input(&expected_input).unwrap();

            let t = Tile::from_input(&input).unwrap();
            assert_eq!(t.rotate_left(), expected);
        }

        #[test]
        fn find_shape() {
            let input = test_input(
                "
                Tile 11111:
                .####...#####..#...###..
                #####..#..#.#.####..#.#.
                .#.#...#.###...#.##.##..
                #.#.##.###.#.##.##.#####
                ..##.###.####..#.####.##
                ...#.#..##.##...#..#..##
                #.##.#..#.#..#..##.#.#..
                .###.##.....#...###.#...
                #.####.#.#....##.#..#.#.
                ##...#..#....#..#...####
                ..#.##...###..#.#####..#
                ....#.##.#.#####....#...
                ..##.##.###.....#.##..#.
                #...#...###..####....##.
                .#.##...#.##.#.#.###...#
                #.###.#..####...##..#...
                #.###...#.##...#.######.
                .###.###.#######..#####.
                ..##.#..#..#.#######.###
                #.#..##.########..#..##.
                #.#####..#.#...##..#....
                #....##..#.#########..##
                #...#.....#..##...###.##
                #..###....##.#...##.##.#
            ",
            );

            let shape = test_input(
                "
                ..................#.
                #....##....##....###
                .#..#..#..#..#..#...
            ",
            )
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

            let mut t = Tile::from_input(&input).unwrap();
            assert_eq!(t.find_shape(&shape), 2);
            assert_eq!(t.count_char('#'), 273);
        }
    }

    mod grid {
        use super::*;

        #[test]
        fn arranging() {
            let input = test_input(
                "
                Tile 2311:
                ..##.#..#.
                ##..#.....
                #...##..#.
                ####.#...#
                ##.##.###.
                ##...#.###
                .#.#.#..##
                ..#....#..
                ###...#.#.
                ..###..###

                Tile 1951:
                #.##...##.
                #.####...#
                .....#..##
                #...######
                .##.#....#
                .###.#####
                ###.##.##.
                .###....#.
                ..#.#..#.#
                #...##.#..

                Tile 1171:
                ####...##.
                #..##.#..#
                ##.#..#.#.
                .###.####.
                ..###.####
                .##....##.
                .#...####.
                #.##.####.
                ####..#...
                .....##...

                Tile 1427:
                ###.##.#..
                .#..#.##..
                .#.##.#..#
                #.#.#.##.#
                ....#...##
                ...##..##.
                ...#.#####
                .#.####.#.
                ..#..###.#
                ..##.#..#.

                Tile 1489:
                ##.#.#....
                ..##...#..
                .##..##...
                ..#...#...
                #####...#.
                #..#.#.#.#
                ...#.#.#..
                ##.#...##.
                ..##.##.##
                ###.##.#..

                Tile 2473:
                #....####.
                #..#.##...
                #.##..#...
                ######.#.#
                .#...#.#.#
                .#########
                .###.#..#.
                ########.#
                ##...##.#.
                ..###.#.#.

                Tile 2971:
                ..#.#....#
                #...###...
                #.#.###...
                ##.##..#..
                .#####..##
                .#..####.#
                #..#.#..#.
                ..####.###
                ..#.#.###.
                ...#.#.#.#

                Tile 2729:
                ...#.#.#.#
                ####.#....
                ..#.#.....
                ....#..#.#
                .##..##.#.
                .#.####...
                ####.#.#..
                ##.####...
                ##..#.##..
                #.##...##.

                Tile 3079:
                #.#.#####.
                .#..######
                ..#.......
                ######....
                ####.#..#.
                .#...#.##.
                #.#####.##
                ..#.###...
                ..#.......
                ..#.###...
            ",
            );

            let mut g = Grid::from_input(&input).unwrap();
            g.arrange();
            assert_eq!(g.get_corner_product().unwrap(), 20899048083289);
        }
    }
}
