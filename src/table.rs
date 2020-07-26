pub struct Table {
    pub width: u8,
    pub height: u8,
    fields: Vec<Option<Figure>>,
}

#[derive(PartialEq, Eq, Debug, Copy, Clone, Hash)]
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

impl Table {
    pub fn new(width: u8, height: u8) -> Table {
        Table {
            width,
            height,
            fields: (0..height * width).map(|_| None).collect(),
        }
    }

    pub fn put(&mut self, col: u8, row: u8, figure: Figure) -> bool {
        let offset = self.offset(col, row);
        let cell = &mut self.fields[offset];
        match cell {
            Some(_) => false,
            free_cell => {
                free_cell.replace(figure);
                true
            }
        }
    }

    fn offset(&self, col: u8, row: u8) -> usize {
        if row >= self.height || col >= self.width {
            panic!("Table index (col:{},row :{}) out of bounds (widht:{},height:{})",
                   col, row, self.width, self.height);
        }
        (row * col) as usize
    }

    pub fn get(&self, col: u8, row: u8) -> Option<Figure> {
        self.fields[self.offset(col, row)]
    }

    pub fn row(&self, row: u8) -> &[Option<Figure>] {
        self.fields.as_slice()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_table() {
        let table = Table::new(12, 10);
        assert_eq!(None, table.get(0, 0));
        assert_eq!(None, table.get(11, 9));
    }

    #[test]
    #[should_panic]
    fn size() {
        let table = Table::new(10, 10);
        table.get(10, 10);
    }

    #[test]
    fn put_to_free_cell() {
        let mut table = Table::new(10, 10);
        assert!(table.put(5, 5, Figure::X));
        assert_eq!(Some(Figure::X), table.get(5, 5));
    }

    #[test]
    fn put_to_owned_cell() {
        let mut table = Table::new(10, 10);
        assert!(table.put(5, 5, Figure::X));
        assert!(!table.put(5, 5, Figure::X));
    }
}
