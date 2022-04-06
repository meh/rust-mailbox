//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//                    Version 2, December 2004
//
// Copyleft (ↄ) meh. <meh@schizofreni.co> | http://meh.schizofreni.co
//
// Everyone is permitted to copy and distribute verbatim or modified
// copies of this license document, and changing it is allowed as long
// as the name is changed.
//
//            DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
//   TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
//
//  0. You just DO WHAT THE FUCK YOU WANT TO.

use crate::header::Header;
use crate::stream::entry::{self, header};
use casing::Casing;
use fnv::FnvHasher;
use std::collections::{hash_map, HashMap};
use std::hash::BuildHasherDefault;
use std::io;

#[derive(Clone, Default, Debug)]
pub struct Headers(HashMap<header::Item, Vec<header::Item>, BuildHasherDefault<FnvHasher>>);

impl Headers {
    #[inline]
    pub(crate) fn insert(&mut self, header: entry::Header) {
        self.0
            .entry(header.key())
            .or_insert_with(Vec::new)
            .push(header.value());
    }

    #[inline]
    pub fn get<H: Header>(&self) -> Option<io::Result<H>> {
        self.0.get(H::name()).map(|v| H::parse(v.as_ref()))
    }

    #[inline]
    pub fn get_from<H: Header, T: AsRef<str>>(&self, key: T) -> Option<io::Result<H>> {
        self.0
            .get(key.as_ref().header(Default::default()).as_ref())
            .map(|v| H::parse(v.as_ref()))
    }

    #[inline]
    pub fn get_raw<T: AsRef<str>>(&self, key: T) -> Option<&[header::Item]> {
        self.0
            .get(key.as_ref().header(Default::default()).as_ref())
            .map(|v| v.as_ref())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn contains_key<T: AsRef<str>>(&self, key: T) -> bool {
        self.0
            .contains_key(key.as_ref().header(Default::default()).as_ref())
    }

    #[inline]
    pub fn contains<H: Header>(&self) -> bool {
        self.0.contains_key(H::name())
    }

    #[inline]
    pub fn keys(&self) -> hash_map::Keys<header::Item, Vec<header::Item>> {
        self.0.keys()
    }

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter(self.0.iter())
    }
}

pub struct HeaderView<'a> {
    key: &'a header::Item,
    values: &'a [header::Item],
}

impl<'a> HeaderView<'a> {
    #[inline]
    pub fn is<H: Header>(&self) -> bool {
        self.key.as_ref() == H::name()
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.key.as_ref()
    }

    #[inline]
    pub fn value<H: Header>(&self) -> io::Result<H> {
        H::parse(self.values)
    }

    #[inline]
    pub fn raw(&self) -> &[header::Item] {
        self.values
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = HeaderView<'a>;
    type IntoIter = Iter<'a>;

    #[inline]
    fn into_iter(self) -> Iter<'a> {
        self.iter()
    }
}

pub struct Iter<'a>(hash_map::Iter<'a, header::Item, Vec<header::Item>>);

impl<'a> Iterator for Iter<'a> {
    type Item = HeaderView<'a>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if let Some((key, values)) = self.0.next() {
            Some(HeaderView { key, values })
        } else {
            None
        }
    }
}
