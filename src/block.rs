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

//! A `Block` implementation.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use chrono::{DateTime, Utc};

/// A `Block` representation.
#[derive(Clone, Debug, Hash, PartialEq)]
pub struct Block<T: Hash> {
    /// An height identifier of the given `Block`.
    pub id: u64,

    /// A binding of the given `Block`.
    pub value: T,

    /// An identifier of the previous `Block` in the [`Chain`].
    ///
    /// [`Chain`]: struct.Chain.html
    pub previous_id: u64,

    /// A timespec that the given `Block` was created.
    pub timestamp: DateTime<Utc>,
}

impl<T: Hash> Block<T> {
    /// Creates a new `Block`.
    pub fn new(id: u64, value: T, previous_id: u64) -> Block<T> {
        let timestamp = Utc::now();

        Block {
            id,
            value,
            previous_id,
            timestamp,
        }
    }

    /// Returns `true` if the given `Block` is trusty for the next `Block`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libblockchain::Block;
    ///
    /// let block0 = Block::new(0, vec![0; 256], 0);
    /// let block1 = block0.next(vec![1, 2]);
    ///
    /// assert!(block1.is_trusty_for(&block0))
    /// ```
    pub fn is_trusty_for(&self, other: &Block<T>) -> bool {
        self.previous_id == gen_id(other)
    }

    /// Constructs the next `Block`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libblockchain::Block;
    ///
    /// let block0 = Block::new(0, vec![0; 256], 0);
    /// let block1 = block0.next(vec![1, 2]);
    ///
    /// println!("{:?}", block1)
    /// ```
    pub fn next(&self, value: T) -> Block<T> {
        let id = self.id + 1;

        let previous_id = gen_id(&self);

        let timestamp = Utc::now();

        Block {
            id,
            value,
            previous_id,
            timestamp,
        }
    }
}

fn gen_id<T: Hash>(t: &T) -> u64 {
    let mut sip = DefaultHasher::default();

    t.hash(&mut sip);

    sip.finish()
}
