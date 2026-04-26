pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod solution {
    use std::ops::{Index, IndexMut, Deref};

    pub struct CircularBuffer<T> {
        buffer: Vec<Option<T>>,
        pub head: usize,
        pub tail: usize,
        capacity: usize,
        count: usize,
    }

    #[derive(Debug, PartialEq)]
    pub enum CircularBufferError {
        BufferFull,
        NotContiguous,
    } 

    impl<T> CircularBuffer<T> {
        pub fn new(capacity: usize) -> Self {
            let mut buffer = Vec::with_capacity(capacity);
            for _ in 0..capacity {
                buffer.push(None);
            }

            Self {
                buffer,
                head: 0,
                tail: 0,
                capacity,
                count: 0,
            }
        }

        pub fn write(&mut self, item: T) -> Result<(), CircularBufferError> {
            
            if self.count == self.capacity {    // if the buffer is full, returns an error
                return Err(CircularBufferError::BufferFull);
            }

            self.buffer[self.tail] = Some(item);
            self.tail = (self.tail + 1) % self.capacity;
            self.count += 1;

            Ok(())
        } 

        pub fn read(&mut self) -> Option<T> {
            
            if self.count == 0 {
                return None;
            }

            let result = self.buffer[self.head].take();
            self.head = (self.head + 1) % self.capacity;
            self.count -= 1;

            return result;
        }

        pub fn clear(&mut self) {
            for i in 0..self.capacity {
                self.buffer[i] = None;
            }

            self.head = 0;
            self.tail = 0;
            self.count = 0;
        }

        pub fn size(&self) -> usize {
            self.count
        }

        pub fn overwrite(&mut self, item: T) {

            if self.count == self.capacity {
                self.buffer[self.tail] = Some(item);
                self.tail = (self.tail + 1) % self.capacity;
                self.head = (self.head + 1) % self.capacity;
            } else {
                let _ = self.write(item);
            }
        }

        pub fn make_contiguous(&mut self) {
            
            if self.count == 0 || self.head == 0 {
                return;
            }

            self.buffer.rotate_left(self.head);
            self.head = 0;
            self.tail = self.count % self.capacity;
        }
    }

    impl<T> Index<usize> for CircularBuffer<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            if index >= self.count {
                panic!("Index out of bounds!");
            }

            let physical_index = (self.head + index) % self.capacity;
            self.buffer[physical_index].as_ref().unwrap()
        }
    }

    impl<T> IndexMut<usize> for CircularBuffer<T> {
        fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            if index >= self.count {
                panic!("Index out of bounds!");
            }

            let physical_index = (self.head + index) % self.capacity;
            self.buffer[physical_index].as_mut().unwrap()
        }
    }
 
    impl<T> Deref for CircularBuffer<T> {
        type Target = [Option<T>];

        fn deref(&self) -> &Self::Target {
            if self.tail < self.head {
                panic!("The circular buffer is not contiguous!");
            }

            &self.buffer[self.head .. self.tail]
        }
    }

    pub trait TryDeref {
        type Target: ?Sized;
        type Error;

        fn try_deref(&self) -> Result<&Self::Target, Self::Error>;
    }

    impl<T> TryDeref for CircularBuffer<T> {
        type Target = [Option<T>];
        type Error = CircularBufferError;

        fn try_deref(&self) -> Result<&Self::Target, Self::Error> {
            if self.tail < self.head {
                return Err(CircularBufferError::NotContiguous);
            }

            Ok(&self.buffer[self.head .. self.tail])
        }
    }
}
