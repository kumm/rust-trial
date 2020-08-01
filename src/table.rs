use std::ops::{Index, IndexMut};

pub struct Table {
    col_count: u8,
    row_count: u8,
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
    pub row: u8,
    pub col: u8,
}

impl Cell {
    fn offset(&self, table: &Table) -> usize {
        if self.row >= table.row_count || self.col >= table.col_count {
            panic!("Table index (col:{},row :{}) out of bounds (col_count:{},row_count:{})",
                   self.col, self.row, table.col_count, table.row_count);
        }
        (self.row * table.col_count + self.col) as usize
    }
}

impl Table {
    pub fn new(col_count: u8, row_count: u8) -> Table {
        Table {
            col_count,
            row_count,
            values: (0..row_count * col_count).map(|_| None).collect(),
        }
    }

    pub fn put(&mut self, cell: Cell, figure: Figure) -> bool {
        let value = &mut self[cell];
        match value {
            Some(_) => false,
            free_cell => {
                free_cell.replace(figure);
                true
            }
        }
    }

    pub fn row_count(&self) -> u8 {
        self.row_count
    }

    pub fn col_count(&self) -> u8 {
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
    row: u8,
    table: &'a Table,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a [CellValue];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.table.row_count {
            let values = &self.table[self.row];
            self.row = self.row + 1;
            return Some(values);
        } else {
            return None;
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = (self.table.row_count - self.row) as usize;
        (remaining, Some(remaining))
    }
}

impl Index<u8> for Table  {
    type Output = [CellValue];

    fn index(&self, row: u8) -> &Self::Output {
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
        let table = Table::new(12, 10);
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
        assert!(table.put(Cell { row: 5, col: 5 }, Figure::X));
        assert_eq!(Some(Figure::X), table[Cell {row: 5, col: 5}]);
    }

    #[test]
    fn cell() {
        let mut table = Table::new(10, 10);
        assertCell(&mut table, 0, 0);
        assertCell(&mut table, 1, 0);
        assertCell(&mut table, 9, 9);
        assertCell(&mut table, 0, 9);
    }

    fn assertCell(table: &mut Table, col: u8, row: u8) {
        assert_eq!(None, table[Cell {col, row}]);
        assert!(table.put(Cell {col, row}, Figure::X));
        assert_eq!(Some(Figure::X), table[Cell {col, row}]);
    }

    #[test]
    fn put_to_owned_cell() {
        let mut table = Table::new(10, 10);
        assert!(table.put(Cell { row: 5, col: 5 }, Figure::X));
        assert!(!table.put(Cell { row: 5, col: 5 }, Figure::X));
    }

    #[test]
    fn iter() {
        let mut table = Table::new(10, 10);
        table.put(Cell { row: 0, col: 4 }, Figure::X);
        let row0 = table.iter().nth(0).unwrap();
        let cell04 = row0.iter().nth(4).unwrap();
        assert_eq!(&Some(Figure::X), cell04);

        let mut rows = 0;
        table.iter().for_each(|_row| rows = rows + 1);
        assert_eq!(10, rows);
    }
}
