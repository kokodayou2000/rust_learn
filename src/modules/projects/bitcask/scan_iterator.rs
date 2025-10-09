use crate::modules::projects::bitcask::log::Log;
use crate::modules::projects::bitcask::common::Result;
use std::collections::btree_map;

// 迭代器实现
pub struct ScanIterator<'a> {
    pub(crate) inner: btree_map::Range<'a, Vec<u8>, (u64, u32)>,
    pub(crate) log: &'a mut Log,
}

impl<'a> ScanIterator<'a> {
    fn map(&mut self, item: (&Vec<u8>, &(u64, u32))) -> <Self as Iterator>::Item {
        let (key, (value_pos, value_len)) = item;
        let value = self.log.read_value(*value_pos, *value_len)?;
        Ok((key.clone(), value))
    }
}

impl<'a> Iterator for ScanIterator<'a> {
    type Item = Result<(Vec<u8>, Vec<u8>)>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|item| self.map(item))
    }
}

impl<'a> DoubleEndedIterator for ScanIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back().map(|item| self.map(item))
    }
}