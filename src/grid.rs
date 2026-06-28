use std::ops::{Index, IndexMut};
use std::thread;

pub struct Grid<T>
where
    T: Send + Sync,
{
    pub vec: Vec<T>,
    pub width: usize,
    pub heigth: usize,
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
    T: Send + Sync,
{
    pub fn new(width: usize, heigth: usize) -> Self
    where
        T: Clone + Default,
    {
        let count = width * heigth;
        Self {
            vec: vec![T::default(); count],
            width,
            heigth,
            count,
        }
    }

    pub fn update_chunks(
        new_grid: &mut Grid<T>,
        old_grid: &Grid<T>,
        chunk_count: usize,
        update: fn(GridPosition, &Grid<T>) -> T,
    ) {
        let chunk_size = new_grid.count / chunk_count;

        thread::scope(|s| {
            for (chunk_index, chunk) in new_grid.vec.chunks_mut(chunk_size).enumerate() {
                s.spawn(move || {
                    let chunk_start_pos = chunk_index * chunk_size;
                    for (cell_index, x) in chunk.iter_mut().enumerate() {
                        *x = update(
                            GridPosition {
                                value: chunk_start_pos + cell_index,
                            },
                            old_grid,
                        );
                    }
                });
            }
        });
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
            && new_row < self.heigth as isize
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

    pub fn create_position(&self, row: usize, col: usize) -> GridPosition {
        GridPosition {
            value: row * self.width + col,
        }
    }
}

impl<T> Index<GridPosition> for Grid<T>
where
    T: Send + Sync,
{
    type Output = T;

    fn index(&self, index: GridPosition) -> &Self::Output {
        &self.vec[index.value]
    }
}

impl<T> IndexMut<GridPosition> for Grid<T>
where
    T: Send + Sync,
{
    fn index_mut(&mut self, index: GridPosition) -> &mut Self::Output {
        &mut self.vec[index.value]
    }
}
