use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum CoinLockerError<T = ()> {
    FullAndReturn(T),
    OutOfBounds,
    NoItemFound,
}

impl Display for CoinLockerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CoinLockerError::FullAndReturn(_) => write!(f, "CoinLocker is full"),
            CoinLockerError::OutOfBounds => write!(f, "Index out of bounds"),
            CoinLockerError::NoItemFound => write!(f, "No item found in CoinLocker"),
        }
    }
}

impl std::error::Error for CoinLockerError {}

pub struct CoinLocker<T> {
    locker: Vec<Option<T>>,
    free_index_stack: Vec<usize>,
}

impl<T> CoinLocker<T> {
    pub fn new(size: usize) -> Self {
        let mut locker = Vec::with_capacity(size);
        let mut free_index_stack = Vec::with_capacity(size);
        for i in (0..size).rev() {
            locker.push(None);
            free_index_stack.push(i);
        }

        CoinLocker {
            locker,
            free_index_stack,
        }
    }

    pub fn insert(&mut self, item: T) -> Result<usize, CoinLockerError<T>> {
        let Some(index) = self.free_index_stack.pop() else {
            // return ownership of item
            return Err(CoinLockerError::FullAndReturn(item));
        };
        assert!(self.locker[index].is_none());
        self.locker[index] = Some(item);
        Ok(index)
    }

    pub fn take(&mut self, index: usize) -> Result<T, CoinLockerError> {
        if index >= self.locker.len() {
            return Err(CoinLockerError::OutOfBounds);
        }

        if self.locker[index].is_none() {
            return Err(CoinLockerError::NoItemFound);
        }
        let item = self.locker[index].take().unwrap();
        self.free_index_stack.push(index);
        Ok(item)
    }

    pub fn clear(&mut self) {
        self.free_index_stack.clear();
        for i in (0..self.locker.len()).rev() {
            self.locker[i] = None;
            self.free_index_stack.push(i);
        }
    }

    pub fn free_count(&self) -> usize {
        self.free_index_stack.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut locker = CoinLocker::new(4);

        assert_eq!(locker.take(4), Err(CoinLockerError::OutOfBounds));
        assert_eq!(locker.take(3), Err(CoinLockerError::NoItemFound));

        assert_eq!(locker.insert("a"), Ok(0));
        assert_eq!(locker.insert("b"), Ok(1));

        assert_eq!(locker.free_count(), 2);
        assert_eq!(locker.take(1), Ok("b"));

        assert_eq!(locker.insert("c"), Ok(1));
        assert_eq!(locker.insert("d"), Ok(2));
        assert_eq!(locker.insert("e"), Ok(3));

        assert_eq!(locker.insert("f"), Err(CoinLockerError::FullAndReturn("f")));

        locker.clear();
        assert_eq!(locker.free_count(), 4);
        assert_eq!(locker.insert("a"), Ok(0));        
    }
}
