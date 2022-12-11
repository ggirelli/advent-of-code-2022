use std::fmt;

#[derive(Debug)]
pub struct Modulus {
    pub modulus: u64,
    pub remainder: u64,
}

impl Modulus {
    pub fn new(n: &u64, modulus: u64) -> Modulus {
        Modulus {
            modulus: modulus,
            remainder: (n % modulus),
        }
    }

    fn _update(&mut self) {
        self.remainder %= self.modulus;
    }

    pub fn multiply(&mut self, n: u64) {
        self.remainder *= n;
        self._update();
    }

    pub fn add(&mut self, n: u64) {
        self.remainder += n;
        self._update();
    }
}

impl PartialEq for Modulus {
    fn eq(&self, other: &Self) -> bool {
        (self.modulus == other.modulus) & (self.remainder == other.remainder)
    }
}

impl fmt::Display for Modulus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} mod({})", self.remainder, self.modulus)
    }
}

#[test]
fn test_item() {
    let mut item: Modulus;
    item = Modulus::new(&28, 5);
    assert_eq!(item, Modulus::new(&28, 5));

    item.add(5);
    assert_eq!(item, Modulus::new(&33, 5));

    item.add(8);
    assert_eq!(item, Modulus::new(&41, 5));

    item = Modulus::new(&28, 5);
    item.multiply(5);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 0);

    item = Modulus::new(&28, 5);
    item.multiply(3);
    assert_eq!(item.modulus, 5);
    assert_eq!(item.remainder, 4);

    item = Modulus::new(&28, 5);
    item.multiply(8);
    assert_eq!(item, Modulus::new(&(28 * 8), 5));
}
