use std::cell::RefCell;

pub struct Countdown {
    value: RefCell<usize>,
}

impl Countdown {
    pub fn new(from: usize) -> Countdown {
        Countdown {
            value: RefCell::new(from),
        }
    }

    pub fn step(&self) -> Option<usize> {
        let last: usize = *self.value.borrow();
        if last == 0 {
            None
        } else {
            let mut value = self.value.borrow_mut();
            *value = last - 1;
            Some(last - 1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn countdown() {
        let countdown = Countdown::new(3);
        assert_eq!(countdown.step(), Some(2));
        assert_eq!(countdown.step(), Some(1));
        assert_eq!(countdown.step(), Some(0));
        assert_eq!(countdown.step(), None);
    }

    #[test]
    fn allows_multiple_immutable_borrows() {
        let x = RefCell::new(3);
        let y = x.borrow();
        let z = x.borrow();
        assert_eq!(*y, 3);
        assert_eq!(*z, 3);
    }

    #[test]
    fn allows_single_mutable_borrow() {
        let x = RefCell::new(3);
        let mut y = x.borrow_mut();
        *y = 4;
        assert_eq!(*y, 4);
    }

    #[test]
    #[should_panic(expected = "already borrowed")]
    fn disallow_multiple_mutable_borrows() {
        let x = RefCell::new(3);
        let _y = x.borrow_mut();
        let _z = x.borrow_mut();
    }
}
