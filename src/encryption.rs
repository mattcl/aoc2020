const DIVISOR: usize = 20201227;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Key(pub usize);

impl Key {
    pub fn new(loop_size: usize, subject_number: usize) -> Self {
        let mut value = 1;

        for _ in 0..loop_size {
            value *= subject_number;
            value = value % DIVISOR;
        }

        Key(value)
    }

    pub fn loop_size(&self, subject_number: usize) -> usize {
        let mut value = 1;
        let mut size = 0;
        while value != self.0 {
            value *= subject_number;
            value = value % DIVISOR;
            size += 1;
        }
        size
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Device {
    pub public_key: Key,
    loop_size: usize,
}

impl Device {
    pub fn new(loop_size: usize, subject_number: usize) -> Self {
        Device {
            public_key: Key::new(loop_size, subject_number),
            loop_size: loop_size,
        }
    }

    pub fn from_key(key: Key, subject_number: usize) -> Self {
        let loop_size = key.loop_size(subject_number);
        Device {
            public_key: key,
            loop_size: loop_size,
        }
    }

    pub fn encryption_key(&self, other: &Key) -> Key {
        Key::new(self.loop_size, other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod key {
        use super::*;

        #[test]
        fn creation() {
            assert_eq!(Key::new(8, 7), Key(5764801));
            assert_eq!(Key::new(11, 7), Key(17807724));
        }

        #[test]
        fn derriving_loop_size() {
            assert_eq!(Key(5764801).loop_size(7), 8);
            assert_eq!(Key(17807724).loop_size(7), 11);
        }
    }

    mod device {
        use super::*;

        #[test]
        fn from_key() {
            assert_eq!(Device::from_key(Key(5764801), 7), Device::new(8, 7));
            assert_eq!(Device::from_key(Key(17807724), 7), Device::new(11, 7));
        }

        #[test]
        fn encryption_key() {
            let d1 = Device::new(8, 7);
            let d2 = Device::new(11, 7);

            assert_eq!(d1.encryption_key(&d2.public_key), Key(14897079));
            assert_eq!(d2.encryption_key(&d1.public_key), Key(14897079));
        }
    }
}
