/// Internal struct for iterating over input bytes
#[derive(Debug)]
pub struct Stream<'a, T> {
    pub idx: usize,
    data: &'a [T],
}

impl<'a, T: Copy> Stream<'a, T> {
    /// Returns a copy of the current element
    #[inline]
    pub fn current_cpy(&self) -> Option<T> {
        self.data.get(self.idx).copied()
    }
}

impl<'a, T: Eq + Copy> Stream<'a, T> {
    /// Increases internal index by 1 if the given element matches the current element
    /// If it does match, the expected character is returned
    #[inline]
    pub fn expect_and_skip(&mut self, expect: T) -> Option<T> {
        let c = self.current_cpy()?;
        if c == expect {
            self.advance();
            Some(c)
        } else {
            None
        }
    }

    /// Increases internal index by 1 if any of the given elements match the current element
    /// If it does match, the expected character is returned
    pub fn expect_oneof_and_skip(&mut self, expect: &[T]) -> Option<T> {
        let c = self.current_cpy()?;

        if expect.contains(&c) {
            self.advance();
            return Some(c);
        }

        None
    }

    /// Same as expect_and_skip, but returns a bool
    #[inline]
    pub fn expect_and_skip_cond(&mut self, expect: T) -> bool {
        self.expect_and_skip(expect).is_some()
    }
}

impl<'a, T> Stream<'a, T> {
    /// Creates a new stream
    #[inline]
    pub fn new(data: &'a [T]) -> Stream<'a, T> {
        Self { data, idx: 0 }
    }

    #[inline]
    pub fn advance(&mut self) {
        self.idx += 1;
    }

    /// Returns the current element
    #[inline]
    pub fn current(&self) -> Option<&T> {
        self.data.get(self.idx)
    }

    /// Checks whether the stream has reached the end
    #[inline]
    pub fn is_eof(&self) -> bool {
        self.idx >= self.data.len()
    }

    /// Returns a subslice of this stream, and panicks if out of bounds
    #[inline]
    pub fn slice(&self, from: usize, to: usize) -> &'a [T] {
        &self.data[from..to]
    }
}
