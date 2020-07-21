pub struct DiamondIterator {
    side: Side,
    center_x: i32,
    center_y: i32,
    max_radius: u16,
    radius: u16,
    x: i32,
    y: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum Side {
    TopRight,
    BottomRight,
    BottomLeft,
    TopLeft,
}

impl DiamondIterator {
    pub fn new(center_x: i32, center_y: i32, max_radius: u16) -> Self {
        Self {
            side: Side::TopRight,
            y: -2,
            x: -1,
            radius: 1,
            center_x,
            center_y,
            max_radius,
        }
    }
}

impl Iterator for DiamondIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        match self.side {
            Side::TopRight => {
                self.y += 1;
                self.x += 1;
                if self.y == 0 {
                    self.side = Side::BottomRight;
                }
            }
            Side::BottomRight => {
                self.y += 1;
                self.x -= 1;
                if self.x == 0 {
                    self.side = Side::BottomLeft;
                }
            }
            Side::BottomLeft => {
                self.y -= 1;
                self.x -= 1;
                if self.y == 0 {
                    self.side = Side::TopLeft;
                }
            }
            Side::TopLeft => {
                self.y -= 1;
                self.x += 1;
                if self.x == 0 {
                    self.side = Side::TopRight;
                    self.radius += 1;
                    self.x = 0;
                    self.y = -(self.radius as i32);
                }
            }
        }

        if self.radius == self.max_radius + 1 {
            None
        } else {
            Some((self.center_x + self.x, self.center_y + self.y))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn diamond_1() {
        let mut d = DiamondIterator::new(2, 2, 1);
        let mut s = [','; 5 * 5].iter().collect::<String>();
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,,,\
             ,,,,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,O,\
             ,,,,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,O,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        assert_eq!(d.next(), None);
    }

    #[test]
    fn diamond_2() {
        let mut d = DiamondIterator::new(2, 2, 2);
        let mut s = [','; 5 * 5].iter().collect::<String>();
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,,,\
             ,,,,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,O,\
             ,,,,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,,,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,,,,\
             ,,O,,\
             ,O,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,O,,\
             ,O,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             ,O,O,\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             ,O,OO\
             ,,O,,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             ,O,OO\
             ,,OO,\
             ,,,,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             ,O,OO\
             ,,OO,\
             ,,O,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             ,O,OO\
             ,OOO,\
             ,,O,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,,OO,\
             OO,OO\
             ,OOO,\
             ,,O,,"
                .to_string()
        );
        add_pos(&mut s, d.next());
        assert_eq!(
            s,
            ",,O,,\
             ,OOO,\
             OO,OO\
             ,OOO,\
             ,,O,,"
                .to_string()
        );
        assert_eq!(d.next(), None);
    }

    fn add_pos(s: &mut String, pos: Option<(i32, i32)>) {
        if let Some((x, y)) = pos {
            let index = x as usize + y as usize * 5;
            let mut new = String::new();
            for (i, mut c) in s.chars().enumerate() {
                if i == index {
                    c = 'O'
                }
                new.push(c);
            }
            *s = new;
        }
    }
}
