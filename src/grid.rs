use std::ops::{Index, IndexMut};

#[derive(Clone)]
pub struct Grid<T>
where
    T: Send + Sync + Copy + Default,
{
    pub vec: Vec<T>,
    pub width: usize,
    pub height: usize,
    pub count: usize,
}
pub struct GridPosition {
    value: usize,
}
pub struct GridOffset {
    row: isize,
    col: isize,
}

pub const NEIGHBOUR_OFFSETS: [GridOffset; 8] = [
    GridOffset { row: -1, col: -1 },
    GridOffset { row: -1, col: 0 },
    GridOffset { row: -1, col: 1 },
    GridOffset { row: 0, col: -1 },
    GridOffset { row: 0, col: 1 },
    GridOffset { row: 1, col: -1 },
    GridOffset { row: 1, col: 0 },
    GridOffset { row: 1, col: 1 },
];

impl<T> Grid<T>
where
    T: Send + Sync + Copy + Default,
{
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        Self {
            vec: vec![T::default(); count],
            width,
            height,
            count,
        }
    }

    pub fn add_offset_to_position(
        &self,
        position: &GridPosition,
        offset: &GridOffset,
    ) -> Option<GridPosition> {
        let row = position.value / self.width;
        let col = position.value % self.width;

        let new_row = row as isize + offset.row;
        let new_col = col as isize + offset.col;

        if new_row >= 0
            && new_row < self.height as isize
            && new_col >= 0
            && new_col < self.width as isize
        {
            Some(GridPosition {
                value: (new_row as usize) * self.width + (new_col as usize),
            })
        } else {
            None
        }
    }

    pub fn create_position(&self, row: usize, col: usize) -> Option<GridPosition> {
        self.create_position_index(row * self.width + col)
    }
    pub fn create_position_index(&self, index: usize) -> Option<GridPosition> {
        if index < self.count {
            Some(GridPosition { value: index })
        } else {
            None
        }
    }

    pub fn load_grid(&mut self, vec: Vec<Vec<T>>, origin_position: GridPosition) {
        for (x_off, row) in vec.iter().enumerate() {
            for (y_off, cell) in row.iter().enumerate() {
                let offset = GridOffset {
                    row: x_off as isize,
                    col: y_off as isize,
                };
                let new_pos = self.add_offset_to_position(&origin_position, &offset);
                match new_pos {
                    Some(new) => self[new] = *cell,
                    _ => (),
                }
            }
        }
    }
}

impl<T> Index<GridPosition> for Grid<T>
where
    T: Send + Sync + Copy + Default,
{
    type Output = T;

    fn index(&self, index: GridPosition) -> &Self::Output {
        &self.vec[index.value]
    }
}

impl<T> IndexMut<GridPosition> for Grid<T>
where
    T: Send + Sync + Copy + Default,
{
    fn index_mut(&mut self, index: GridPosition) -> &mut Self::Output {
        &mut self.vec[index.value]
    }
}
