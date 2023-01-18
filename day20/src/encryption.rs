use std::convert::Infallible;
use std::ptr;
use std::str::FromStr;

pub struct File {
    entries: Vec<Link>,
    zero: Link,
}

type Link = *mut Entry;

struct Entry {
    next: Link,
    prev: Link,
    number: i64,
}

impl File {
    pub fn decrypt(&self, key: i64) {
        unsafe {
            for &entry in &self.entries {
                (*entry).number *= key;
            }
        }
        for _ in 0..10 {
            self.mix();
        }
    }

    pub fn mix(&self) {
        let max = i64::try_from(self.entries.len()).unwrap() - 1;
        unsafe {
            for &entry in &self.entries {
                Entry::mix(entry, (*entry).number % max);
            }
        }
    }

    pub fn get(&self, index: usize) -> i64 {
        let mut entry = self.zero;
        unsafe {
            for _ in 0..index % self.entries.len() {
                entry = (*entry).next;
            }
            (*entry).number
        }
    }
}

impl Entry {
    fn mix(entry: Link, amount: i64) {
        if amount == 0 {
            return;
        }
        unsafe {
            let mut target = entry;
            if amount < 0 {
                for _ in amount..=0 {
                    target = (*target).prev;
                }
            } else {
                for _ in 0..amount {
                    target = (*target).next;
                }
            }
            Self::move_to(entry, target);
        }
    }

    #[allow(clippy::missing_const_for_fn)]
    unsafe fn move_to(from: Link, target: Link) {
        let Self { prev, next, .. } = *from;
        let new_next = (*target).next;
        (*prev).next = next;
        (*next).prev = prev;
        (*target).next = from;
        (*from).prev = target;
        (*from).next = new_next;
        (*new_next).prev = from;
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            self.zero = ptr::null_mut();
            while let Some(entry) = self.entries.pop() {
                drop(Box::from_raw(entry));
            }
        }
    }
}

impl FromStr for File {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(s.trim().lines().map(|line| line.parse().unwrap()).into())
    }
}

#[allow(clippy::fallible_impl_from)]
impl<I: IntoIterator<Item = i64>> From<I> for File {
    fn from(iter: I) -> Self {
        let mut entries: Vec<_> = iter
            .into_iter()
            .map(|number| {
                Box::into_raw(Box::new(Entry {
                    next: ptr::null_mut(),
                    prev: ptr::null_mut(),
                    number,
                }))
            })
            .collect();
        let last_index = entries.len() - 1;
        unsafe {
            (*entries[0]).prev = entries[last_index];
            (*entries[last_index]).next = entries[0];
            for index in 0..last_index {
                (*entries[index]).next = entries[index + 1];
                (*entries[index + 1]).prev = entries[index];
            }
            let zero = *entries.iter().find(|&&entry| (*entry).number == 0).unwrap();
            Self { entries, zero }
        }
    }
}
