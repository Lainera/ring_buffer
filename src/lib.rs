#![no_std]

pub trait Read<Word> {
    type Error;
    fn read(&mut self, buf: &mut [Word]) -> Result<usize, Self::Error>;
}

pub trait ReadExact<Word>: Read<Word> {
    fn read_exact(&mut self, buf: &mut [Word]) -> Result<usize, <Self as Read<Word>>::Error>;
}

pub trait Write<Word> {
    type Error;
    fn write(&mut self, buf: &[Word]) -> Result<usize, Self::Error>;
}

pub trait WriteAll<Word>: Write<Word> {
    fn write_all(&mut self, buf: &[Word]) -> Result<usize, <Self as Write<Word>>::Error>;
}

pub struct RingBuffer<'a, Word> {
    store: &'a mut [Word],
    read: usize,
    write: usize,
}


impl<'a, Word: Clone> RingBuffer<'a, Word> {
    pub fn new(buf: &'a mut [Word]) -> Result<Self, RingBufferError> {
        if buf.len() % 2 != 0 {
            Err(RingBufferError::InvalidCapacity)
        } else {
            Ok(Self {
                store: buf,
                read: 0,
                write: 0,
            })
        }
    }

    fn mask(&self, ix: usize) -> usize {
        (self.store.len() - 1) & ix
    }

    pub fn len(&self) -> usize {
        self.write - self.read
    }

    pub fn capacity(&self) -> usize {
        self.store.len()
    }

    pub fn is_empty(&self) -> bool {
        self.write == self.read
    }

    pub fn reset(&mut self) {
        self.read = 0;
        self.write = 0;
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.store.len()
    }

    pub fn push(&mut self, item: Word) -> Result<(), RingBufferError> {
        if self.is_full() {
            return Err(RingBufferError::Full);
        }
        self.store[self.mask(self.write)] = item;
        self.write = self.write.wrapping_add(1);
        Ok(())
    }

    pub fn pop(&mut self) -> Result<Word, RingBufferError> {
        if self.is_empty() {
            return Err(RingBufferError::Empty);
        }
        let item = self.store[self.mask(self.read)].clone();
        self.read = self.read.wrapping_add(1);
        Ok(item)
    }
}

impl<'a, Word: core::fmt::Debug> core::fmt::Debug for RingBuffer<'a, Word> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RingBuffer")
            .field("store", &self.store)
            .field("read", &self.read)
            .field("write", &self.write)
            .finish()
    }
}


#[derive(Debug)]
pub enum RingBufferError {
    Full,
    Empty,
    InvalidCapacity,
}

impl core::fmt::Display for RingBufferError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {     
        f.write_str(self.into())
    }
}

impl core::convert::From<RingBufferError> for &'static str {
    fn from(value: RingBufferError) -> Self {
        (&value).into()
    }
}

impl core::convert::From<&RingBufferError> for &'static str {
    fn from(value: &RingBufferError) -> Self {
        match value {
            RingBufferError::Empty => "RE",
            RingBufferError::Full => "RF",
            RingBufferError::InvalidCapacity => "RC", 
        }
    }
}
