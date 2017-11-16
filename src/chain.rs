// The MIT License (MIT)
//
// Copyright (c) 2017 Doublify Technologies
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! A `Chain` implementation.

use std::hash::Hash;

use block::Block;

/// A `Chain` representation.
#[derive(Debug)]
pub struct Chain<T: Clone + Hash> {
    inner: Vec<Block<T>>,
}

impl<T: Clone + Hash> Chain<T> {
    /// Creates a new `Chain`.
    pub fn new(inner: Vec<Block<T>>) -> Chain<T> {
        Chain { inner }
    }

    /// Appends a block to the `Chain`.
    ///
    /// [`None`]: https://doc.rust-lang.org/std/option/enum.Option.html#variant.None
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libblockchain::{Block, Chain};
    ///
    /// let mut chain = Chain::new(vec![]);
    ///
    /// let block0 = Block::new(0, vec![0; 256], 0);
    ///
    /// let resp = chain.push(block0);
    ///
    /// assert_eq!(resp.is_some(), true)
    /// ```
    pub fn push(&mut self, v: Block<T>) -> Option<()> {
        Some(self.inner.push(v))
    }

    /// Returns `true` if the given `Chain` is trusty.
    pub fn is_trusty_chain(&self) -> bool {
        let mut iter = self.inner.clone().into_iter();

        while let Some(previous) = iter.next() {
            if let Some(current) = iter.next() {
                if !current.is_trusty_for(&previous) {
                    return false;
                }
            }
        }

        true
    }
}
