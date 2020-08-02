use std::ops::{Index, IndexMut};

pub struct Table {
    col_count: usize,
    row_count: usize,
    values: Vec<CellValue>,
}
type CellValue = Option<Figure>;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum Figure {
    X = 0,
    O = 1,
}

impl Figure {
    pub fn opponent(&self) -> Figure {
        match self {
            Figure::X => Figure::O,
            Figure::O => Figure::X,
        }
    }
}


#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub struct Cell {
    pub row: usize,
    pub col: usize,
}

impl Cell {
    fn offset(&self, table: &Table) -> usize {
        if !self.is_valid(table) {
            panic!("Table index (col:{},row :{}) out of bounds (col_count:{},row_count:{})",
                   self.col, self.row, table.col_count, table.row_count);
        }
        (self.row * table.col_count + self.col) as usize
    }

    pub fn is_valid(&self, table: &Table) -> bool {
        self.row < table.row_count && self.col < table.col_count
    }
}

impl Table {
    pub fn new(row_count: usize, col_count: usize) -> Table {
        Table {
            col_count,
            row_count,
            values: (0..row_count * col_count).map(|_| None).collect(),
        }
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn col_count(&self) -> usize {
        self.col_count
    }

    pub fn iter(&self) -> Iter {
        Iter {
            row: 0,
            table: self,
        }
    }
}

struct Iter<'a> {
    row: usize,
    table: &'a Table,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [CellValue];

    fn next(&mut self) -> Option<Self::Item> {
        return if self.row < self.table.row_count {
            let values = &self.table[self.row];
            self.row = self.row + 1;
            Some(values)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.table.row_count - self.row) as usize;
        (remaining, Some(remaining))
    }
}

impl Index<usize> for Table  {
    type Output = [CellValue];

    fn index(&self, row: usize) -> &Self::Output {
        let start = Cell { col: 0, row }.offset(self);
        let end = Cell { col: self.col_count - 1, row }.offset(self);
        &self.values[start..end + 1]
    }
}

impl Index<Cell> for Table {
    type Output = CellValue;

    fn index(&self, cell: Cell) -> &Self::Output {
        let offset = cell.offset(self);
        &self.values[offset]
    }
}

impl IndexMut<Cell> for Table {
    fn index_mut(&mut self, cell: Cell) -> &mut Self::Output {
        let offset = cell.offset(self);
        &mut self.values[offset]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let table = Table::new(10, 12);
        assert_eq!(None, table[Cell { row: 0, col: 0 }]);
        assert_eq!(None, table[Cell { row: 9, col: 11 }]);
    }

    #[test]
    #[should_panic]
    fn size() {
        let table = Table::new(10, 10);
        table[Cell { row: 10, col: 10 }];
    }

    #[test]
    fn put_to_free_cell() {
        let mut table = Table::new(10, 10);
        table[Cell {row: 5, col: 5}] = Some(Figure::X);
        assert_eq!(Some(Figure::X), table[Cell {row: 5, col: 5}]);
    }

    #[test]
    fn cell() {
        let mut table = Table::new(10, 10);
        assert_cell(&mut table, 0, 0);
        assert_cell(&mut table, 1, 0);
        assert_cell(&mut table, 9, 9);
        assert_cell(&mut table, 0, 9);
    }

    fn assert_cell(table: &mut Table, col: usize, row: usize) {
        assert_eq!(None, table[Cell {col, row}]);
        table[Cell {col, row}] = Some(Figure::X);
        assert_eq!(Some(Figure::X), table[Cell {col, row}]);
    }

    #[test]
    fn iter() {
        let mut table = Table::new(10, 10);
        table[Cell { row: 0, col: 4 }] = Some(Figure::X);
        let row0 = table.iter().nth(0).unwrap();
        let cell04 = row0.iter().nth(4).unwrap();
        assert_eq!(&Some(Figure::X), cell04);

        let mut rows = 0;
        table.iter().for_each(|_row| rows = rows + 1);
        assert_eq!(10, rows);
    }
}
