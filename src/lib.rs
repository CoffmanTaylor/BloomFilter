use std::{
    collections::{hash_map::RandomState, HashSet},
    hash::{BuildHasher, Hash, Hasher},
    marker::PhantomData,
};

pub struct BloomFilter<T> {
    hash_codes: HashSet<u64>,
    hasher_state: RandomState,
    _tag: PhantomData<fn() -> T>,
}

impl<T> BloomFilter<T> {
    pub fn new() -> BloomFilter<T> {
        BloomFilter {
            hash_codes: HashSet::new(),
            hasher_state: RandomState::new(),
            _tag: PhantomData,
        }
    }

    /// Will insert the given item into the set. Returns true if the item was definitely not already
    /// in the set. Returns false if the item was *MAYBE* already in the set.
    ///
    /// Note: False positives are possible but not false negatives.
    pub fn insert(&mut self, t: &T) -> bool
    where
        T: Hash,
    {
        self.hash_codes.insert(self.get_hash(&t))
    }

    /// Returns true if the set *MIGHT* contain the item. Returns false if the set definitely does not
    /// contain the item. I.e: Can have false positives, but can not have false negatives.
    ///
    /// ```
    /// let mut filter = bloom_filter::BloomFilter::new();
    ///
    /// filter.insert(&1);
    ///
    /// assert!(filter.possibly_contains(&1)); // Always true because you can't have false negatives.
    /// // assert!(!filter.possibly_contains(&2)); // Not always false because you can have false positives.
    /// ```
    pub fn possibly_contains(&self, t: &T) -> bool
    where
        T: Hash,
    {
        self.hash_codes.contains(&self.get_hash(&t))
    }

    fn get_hash(&self, t: &T) -> u64
    where
        T: Hash,
    {
        let mut hasher = self.hasher_state.build_hasher();
        t.hash(&mut hasher);
        hasher.finish()
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    proptest! {
        #[test]
        fn if_added_then_possibly_contains_must_be_true(input: Vec<usize>) {
            let mut filter: BloomFilter<usize> = BloomFilter::new();

            input.iter().for_each(|x| {
                filter.insert(&x);
                ()
            });

            for x in input {
                prop_assert!(filter.possibly_contains(&x));
            }
        }
    }
}
