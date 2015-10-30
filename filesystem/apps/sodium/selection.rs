use super::*;
use redox::*;

impl Editor {
    /// Remove from a given motion (row based)
    pub fn remove_rb<'a>(&mut self, (x, y): (isize, isize)) {
        debugln!("Y: {}, Bounded y: {}", y, self.y());
        if y == self.y() as isize {
            let (x, y) = self.bound((x as usize, y as usize));
            // Single line mode
            let (a, b) = if self.x() > x {
                (x, self.x())
             } else {
                (self.x(), x)
            };
            debugln!("A: {}, B: {}", a, b);
            for _ in a..b {
                self.text[y].remove(a);
            }
        } else {
            let (x, y) = self.bound((x as usize, y as usize));
            debugln!("Full line mode!");
            // Full line mode
            let (a, b) = if self.y() < y {
                (self.y(), y)
            } else {
                (y, self.y())
            };
            for _ in a..(b + 1) {
                if self.text.len() > 1 {
                    self.text.remove(a);
                } else {
                    self.text[0] = VecDeque::new();
                }
            }
        }
    }
}