use std::fmt;

#[derive(Debug)]
pub struct Modulus {
    pub modulus: u128,
    pub factor: u128,
    pub remainder: u128,
}

impl Modulus {
    pub fn new(n: &u128, modulus: u128) -> Modulus {
        Modulus {
            modulus: modulus,
            factor: n / modulus,
            remainder: (n % modulus),
        }
    }

    fn _update(&mut self) {
        self.factor += self.remainder / self.modulus;
        self.remainder %= self.modulus;
    }

    pub fn multiply(&mut self, n: Modulus) {
        self.factor *= n.result();
        self.remainder *= n.result();
        self._update();
    }

    pub fn _multiply_n(&mut self, n: u128) {
        self.factor *= n;
        self.remainder *= n;
        self._update();
    }

    pub fn add(&mut self, n: Modulus) {
        assert_eq!(self.modulus, n.modulus);
        self.factor += n.factor;
        self.remainder += n.remainder;
        self._update();
    }

    pub fn _add_n(&mut self, n: u128) {
        self.remainder += n;
        self._update();
    }

    pub fn divide_n(&mut self, n: u128) {
        if n == self.factor {
            self.factor = 1;
            self.remainder /= n;
        } else if n > self.factor {
            let n2: Modulus = Modulus::new(
                &((self.factor * self.modulus + self.remainder) / n),
                self.modulus,
            );
            self.factor = n2.factor;
            self.remainder = n2.remainder;
        } else {
            self.remainder = (self.remainder + (self.factor % n) * self.modulus) / n;
            self.factor /= n;
        }
    }

    pub fn change_modulus(&mut self, n: u128) {
        if n == self.modulus {
            return;
        } else if n > self.modulus {
            self.remainder += (self.factor % n) * self.modulus;
            self.factor -= self.factor % n;

            self.factor = self.factor / n * self.modulus;
            self.modulus = n;
            self._update();
        } else {
            self.remainder += (self.modulus % n) * self.factor;
            self.modulus = n;
            self._update();
        }
    }

    pub fn remainder_with_modulus(&self, n: u128) -> u128 {
        if n == self.modulus {
            return self.remainder;
        } else if n > self.modulus {
            let mut remainder: u128 = self.remainder;
            remainder += (self.factor % n) * self.modulus;
            return remainder % n;
        } else {
            return (self.remainder + (self.modulus % n) * self.factor) % n;
        }
    }

    pub fn result(&self) -> u128 {
        self.factor * self.modulus + self.remainder
    }
}

impl PartialEq for Modulus {
    fn eq(&self, other: &Self) -> bool {
        (self.modulus == other.modulus)
            & (self.factor == other.factor)
            & (self.remainder == other.remainder)
    }
}

impl fmt::Display for Modulus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}*{}+{}", self.modulus, self.factor, self.remainder)
    }
}

#[test]
fn test_item() {
    let mut item: Modulus;
    item = Modulus::new(&28, 5);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 3);
    assert_eq!(item.factor, 5);

    item._add_n(5);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 3);
    assert_eq!(item.factor, 6);

    item._add_n(8);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 1);
    assert_eq!(item.factor, 8);

    item = Modulus::new(&28, 5);
    item._multiply_n(5);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 0);
    assert_eq!(item.factor, 28);

    item = Modulus::new(&28, 5);
    item._multiply_n(3);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 4);
    assert_eq!(item.factor, 16);

    item = Modulus::new(&28, 5);
    item._multiply_n(8);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 4);
    assert_eq!(item.factor, 44);

    item = Modulus::new(&28, 5);
    item.change_modulus(8);
    assert_eq!(item, Modulus::new(&28, 8));

    item = Modulus::new(&28, 5);
    assert_eq!(item.remainder_with_modulus(8), 4);

    item = Modulus::new(&126, 5);
    item.divide_n(4);
    assert_eq!(item.factor, 6);
    assert_eq!(item.remainder, 1);

    item = Modulus::new(&126, 5);
    item.divide_n(7);
    assert_eq!(item.factor, 3);
    assert_eq!(item.remainder, 3);

    item = Modulus::new(&126, 5);
    item.divide_n(25);
    assert_eq!(item, Modulus::new(&5, 5));

    item = Modulus::new(&126, 5);
    item.divide_n(3);
    assert_eq!(item, Modulus::new(&42, 5));

    item = Modulus::new(&126, 5);
    item.divide_n(26);
    assert_eq!(item, Modulus::new(&4, 5));
}
