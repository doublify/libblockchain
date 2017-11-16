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

extern crate libblockchain;

use libblockchain::{Block, Chain};

#[test]
fn is_trusty_chain_test1() {
    let mut chain = Chain::new(vec![]);

    let block0 = Block::new(0, vec![0; 256], 0);
    let block1 = block0.next(vec![1, 2]);
    let block2 = block1.next(vec![3, 4]);
    let block3 = block2.next(vec![5, 6]);

    chain.push(block0);
    chain.push(block1);
    chain.push(block2);
    chain.push(block3);

    assert!(chain.is_trusty_chain())
}


#[test]
fn is_trusty_chain_test2() {
    let mut chain = Chain::new(vec![]);

    let block0 = Block::new(0, vec![0; 256], 0);
    let block1 = block0.next(vec![1, 2]);
    let block2 = block1.next(vec![3, 4]);

    chain.push(block0);
    chain.push(block2);
    chain.push(block1);

    assert_eq!(chain.is_trusty_chain(), false)
}
