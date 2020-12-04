use crate::error::{AocError, Result};

pub struct Forest {
    layout: Vec<Vec<char>>,
    width: usize,
}

impl Forest {
    pub fn new(spec: &Vec<String>) -> Result<Self> {
        if spec.len() < 1 {
            return Err(AocError::ForestDefinitionError("No rows".to_string()));
        }

        let mut layout: Vec<Vec<char>> = Vec::new();
        let width = spec[0].len();

        if width < 1 {
            return Err(AocError::ForestDefinitionError(
                "No columns in first row".to_string(),
            ));
        }

        for row in spec {
            if row.len() != width {
                return Err(AocError::ForestDefinitionError(
                    "Not all columns are the same length".to_string(),
                ));
            }
            layout.push(row.chars().collect());
        }

        Ok(Forest {
            layout: layout,
            width: width,
        })
    }

    pub fn traverse(&self, row: usize, col: usize) -> usize {
        let mut count = 0;
        let mut c = 0;
        for r in (0..self.layout.len()).step_by(row) {
            if self.layout[r][c] == '#' {
                count += 1;
            }
            c = (c + col) % self.width
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spec() -> Vec<String> {
        vec![
            "..##.......".to_string(),
            "#...#...#..".to_string(),
            ".#....#..#.".to_string(),
            "..#.#...#.#".to_string(),
            ".#...##..#.".to_string(),
            "..#.##.....".to_string(),
            ".#.#.#....#".to_string(),
            ".#........#".to_string(),
            "#.##...#...".to_string(),
            "#...##....#".to_string(),
            ".#..#...#.#".to_string(),
        ]
    }

    fn make_forest() -> Forest {
        Forest::new(&spec()).unwrap()
    }

    #[test]
    fn definition() {
        let f = Forest::new(&spec());
        assert!(f.is_ok());

        let f = Forest::new(&Vec::new());
        assert!(f.is_err());

        let mut spec = spec();
        spec.push(".#".to_string());

        let f = Forest::new(&spec);
        assert!(f.is_err());
    }

    #[test]
    fn traverse() {
        let f = make_forest();
        assert_eq!(f.traverse(1, 3), 7);
    }

    #[test]
    fn traverse_multiple() {
        let f = make_forest();

        let res = vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)]
            .into_iter()
            .map(|(row, col)| f.traverse(row, col))
            .fold(1, |acc, count| acc * count);

        assert_eq!(res, 336);
    }
}
