use crate::BrailleCharTrait;


#[derive(Clone, Debug)]
pub struct BrailleCharGridVector<T: BrailleCharTrait> {
    pub array: Vec<T>,
    pub columns: usize, // make private
    pub rows: usize
}

impl<T: BrailleCharTrait> BrailleCharGridVector<T> {
    pub fn new(columns: usize, rows: usize) -> Self {
        return Self {
            array: vec![T::EMPTY; columns * rows],
            columns: columns,
            rows: rows
        };
    }

    pub const fn width(&self) -> usize {
        return self.columns * 2;
    }

    pub const fn height(&self) -> usize {
        return self.rows * 4;
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        assert!(x < self.width());
        assert!(y < self.height());

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[index(c, r, self.width())].get(c_ as u8, r_ as u8);
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> bool {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[index(c, r, self.width())].get_unchecked(c_ as u8, r_ as u8);
    }

    pub fn get_char(&self, x: usize, y: usize) -> &T {
        assert!(x < self.width());
        assert!(y < self.height());

        return &self.array[index(x, y, self.columns)];
    }

    pub fn get_char_unchecked(&self, x: usize, y: usize) -> &T {
        return &self.array[index(x, y, self.columns)];
    }

    pub fn set(&mut self, x: usize, y: usize, value: bool) {
        assert!(x < self.width());
        assert!(y < self.height());

        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[index(c, r, self.columns)].set(c_ as u8, r_ as u8, value);
    }

    pub fn set_unchecked(&mut self, x: usize, y: usize, value: bool) {
        let c = x.div_euclid(2);
        let c_ = x - c * 2;

        let r = y.div_euclid(4);
        let r_ = y - r * 4;

        return self.array[index(c, r, self.columns)].set(c_ as u8, r_ as u8, value);
    }

    pub fn set_char(&mut self, x: usize, y: usize, value: T) {
        assert!(x < self.columns);
        assert!(y < self.rows);

        self.array[index(x, y, self.columns)] = value;
    }

    pub fn set_char_unchecked(&mut self, x: usize, y: usize, value: T) {
        self.array[index(x, y, self.columns)] = value;
    }

    pub fn get_char_mut(&mut self, x: usize, y: usize) -> &mut T {
        assert!(x < self.width());
        assert!(y < self.height());

        return &mut self.array[index(x, y, self.columns)];
    }

    pub fn get_char_mut_unchecked(&mut self, x: usize, y: usize) -> &mut T {
        return &mut self.array[index(x, y, self.columns)];
    }

    pub fn resize(&mut self, columns: usize, rows: usize, (x, y): (isize, isize), value: T) {
        let mut grid = vec![value; columns * rows];

        std::mem::swap(&mut self.array, &mut grid);

        let a = x.max(0) as usize;
        let b = y.max(0) as usize;
        let c = (columns as isize + x).clamp(0, self.columns as isize) as usize;
        let d = (rows as isize + y).clamp(0, self.rows as isize) as usize;

        let w = c - a;
        let h = d - b;

        let a_ = (-x).max(0) as usize;
        let b_ = (-y).max(0) as usize;

        for j in 0..h {
            let slice = &grid[index(a, b + j, self.columns)..index(c, b + j, self.columns)];

            self.array[index(a_, b_ + j, columns)..index(a_ + w, b_ + j, columns)].copy_from_slice(slice);
        }

        self.columns = columns;
        self.rows = rows;
    }
}

#[inline(always)]
const fn index(x: usize, y: usize, width: usize) -> usize {
    return x + y * width;
}

