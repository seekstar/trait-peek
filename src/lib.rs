/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
 */

use std::iter::{Iterator, Peekable};
use std::ops::DerefMut;

pub trait Peek: Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}
impl<T> Peek for Box<dyn Peek<Item = T> + '_> {
    fn peek(&mut self) -> Option<&Self::Item> {
        Peek::peek(self.deref_mut())
    }
}
impl<I: Iterator> Peek for Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        Peekable::peek(self)
    }
}
impl<I: Iterator> Peek for &mut Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        Peekable::peek(self)
    }
}
