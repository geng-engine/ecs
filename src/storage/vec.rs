use crate::archetype::{SplitFields, StructOfAble};

use super::*;

impl<T> Storage<T> for Vec<T> {
    type Family = VecFamily;
    type Id = usize;
    type IdIter = std::ops::Range<usize>;

    fn insert(&mut self, value: T) -> Self::Id {
        let id = self.len();
        self.push(value);
        id
    }

    fn ids(&self) -> Self::IdIter {
        0..self.len()
    }

    fn get(&self, id: Self::Id) -> Option<&T> {
        self.as_slice().get(id)
    }

    fn get_mut(&mut self, id: Self::Id) -> Option<&mut T> {
        self.as_mut_slice().get_mut(id)
    }

    fn remove(&mut self, id: Self::Id) -> Option<T> {
        (id < self.len()).then(|| self.swap_remove(id))
    }

    fn iter(&self) -> Box<dyn Iterator<Item = (Self::Id, &T)> + '_> {
        Box::new(self.as_slice().iter().enumerate())
    }

    fn iter_mut(&mut self) -> Box<dyn Iterator<Item = (Self::Id, &mut T)> + '_> {
        Box::new(self.as_mut_slice().iter_mut().enumerate())
    }
}

pub struct VecFamily;

impl StorageFamily for VecFamily {
    type Id = usize;
    type IdIter = std::ops::Range<usize>;
    type Storage<T> = Vec<T>;
}

impl<T: SplitFields<VecFamily>> StructOfAble for Vec<T> {
    type Struct = T;
    type Family = VecFamily;
}
