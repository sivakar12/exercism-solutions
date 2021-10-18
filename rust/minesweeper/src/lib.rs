
enum Cell {
    Bomb,
    Empty,
    Number(u32)
}

struct MinesweeperGrid {
    grid: Vec<Vec<Cell>>
}

impl From<&[&str]> for MinesweeperGrid {
    fn from(string_list: &[&str]) -> MinesweeperGrid {
        let grid = string_list.iter().map(|row| {
            (row).chars().map(|c| {
                match c {
                    '*' => Cell::Bomb,
                    '.' => Cell::Empty,
                    n => Cell::Number(n.into()),
                }
            }).collect::<Vec<Cell>>()
        }).collect::<Vec<Vec<Cell>>>();
        MinesweeperGrid {grid: grid}
    }
}

impl From<MinesweeperGrid> for Vec<String> {
    fn from(grid: MinesweeperGrid) -> Vec<String> {
        grid.grid.iter().map(|row| {
            let row_string: String = row.iter().map(|c| {
              match c {
                  Cell::Bomb => String::from("*"),
                  Cell::Empty => String::from(" "),
                  Cell::Number(0) => String::from(" "),
                  Cell::Number(n) => n.to_string()
              }
            }).collect::<String>();
            row_string
        }).collect::<Vec<String>>()
    }
}

fn get_neighboring_cells(x: isize, y: isize) -> Vec<(isize, isize)> {
    vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x - 1, y + 1),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y - 1),
        (x + 1, y),
        (x + 1, y + 1)
    ]
}
impl MinesweeperGrid {
    fn get_at_index(&self, x: usize, y: usize) -> Option<&Cell> {
        self.grid.get(x).and_then(|row| row.get(y))
    }
    pub fn get_bomb_count(&self, x: usize, y: usize) -> u32 {
        let neighboring_indexes = get_neighboring_cells(x as isize, y as isize)
            .iter()
            .filter(|(x, y)| {
                *x >= 0 && *y >= 0 
            })
            .map(|(x, y)| {
                (*x as usize, *y as usize)
            })
            .collect::<Vec<(usize, usize)>>();
        let neighboring_cells: Vec<&Cell> = neighboring_indexes
            .iter().filter_map(|(x, y)| self.get_at_index(*x, *y)).collect();
        let bomb_count = neighboring_cells.iter().map(|c| {
            match c {
                Cell::Bomb => 1,
                _ => 0
            }
        }).sum();
        bomb_count
    }
    pub fn calculate_bomb_counts(&mut self) {
        let new_grid = self.grid.iter().enumerate().map(|(x, row)| {
            row.iter().enumerate().map(|(y, c)| {
                let bomb_count = self.get_bomb_count(x, y);
                let new_cell: Cell = match (bomb_count, c) {
                    (_, Cell::Bomb) => Cell::Bomb,
                    (0, _) => Cell::Empty,
                    (n, _) => Cell::Number(n)
                };
                new_cell
            }).collect::<Vec<Cell>>()
        }).collect::<Vec<Vec<Cell>>>();
        self.grid = new_grid;
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut grid: MinesweeperGrid = minefield.into();
    grid.calculate_bomb_counts();
    grid.into()
}
