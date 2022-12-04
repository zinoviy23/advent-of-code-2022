const HALF: usize = 'z' as usize - 'a' as usize + 1;
const SIZE: usize = HALF + HALF;

#[derive(Copy, Clone, Debug, PartialEq)]
struct Item(char);

impl Item {
    fn as_index(&self) -> usize {
        if ('a'..='z').contains(&self.0) {
            self.0 as usize - 'a' as usize
        } else if ('A'..='Z').contains(&self.0) {
            self.0 as usize - 'A' as usize + HALF
        } else {
            panic!("Unexpected char '{}'", self.0);
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        if (0..=25 as usize).contains(&index) {
            Some(Self(char::from_u32('a' as u32 + index as u32).unwrap()))
        } else if (26..=51 as usize).contains(&index) {
            Some(Self(
                char::from_u32('A' as u32 + index as u32 - HALF as u32).unwrap(),
            ))
        } else {
            None
        }
    }

    fn priority(&self) -> u32 {
        self.as_index() as u32 + 1
    }
}

struct ItemsTypeIterator<'a> {
    items: &'a [u32],
    current_item: Option<usize>,
}

impl<'a> Iterator for ItemsTypeIterator<'a> {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current_item = self.current_item.unwrap_or(0);

        while current_item < self.items.len() && self.items[current_item] == 0 {
            current_item += 1;
        }
        if current_item >= self.items.len() {
            self.current_item = Some(current_item);
            return None;
        }
        let item = Item::from_index(current_item);
        current_item += 1;
        self.current_item = Some(current_item);

        return item;
    }
}

struct Compartment {
    items: [u32; SIZE],
}

impl Compartment {
    fn new(s: &str) -> Self {
        let mut items = [0 as u32; SIZE];
        for item in s.chars().map(|c| Item(c)) {
            items[item.as_index()] += 1;
        }

        Self { items }
    }

    fn iter_types(&self) -> ItemsTypeIterator {
        ItemsTypeIterator {
            items: &self.items,
            current_item: None,
        }
    }

    fn contains(&self, item: &Item) -> bool {
        self.items[item.as_index()] != 0
    }
}

pub struct Rucksack {
    first: Compartment,
    second: Compartment,
}

impl Rucksack {
    pub fn new(s: &str) -> Self {
        let (first, second) = s.split_at(s.len() / 2);
        let first = Compartment::new(first);
        let second = Compartment::new(second);
        Self { first, second }
    }

    pub fn wrong_item_priority(&self) -> u32 {
        for item in self.first.iter_types() {
            if self.second.contains(&item) {
                return item.priority();
            }
        }
        panic!("Rucksack do not contain a wrong item type");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iter_test() {
        let compartment = Compartment::new("aabefzAbDZo");
        let mut iter = compartment.iter_types();

        assert_eq!(iter.next(), Some(Item('a')));
        assert_eq!(iter.next(), Some(Item('b')));
        assert_eq!(iter.next(), Some(Item('e')));
        assert_eq!(iter.next(), Some(Item('f')));
        assert_eq!(iter.next(), Some(Item('o')));
        assert_eq!(iter.next(), Some(Item('z')));
        assert_eq!(iter.next(), Some(Item('A')));
        assert_eq!(iter.next(), Some(Item('D')));
        assert_eq!(iter.next(), Some(Item('Z')));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn contains_test() {
        let compartment = Compartment::new("keKlOLAzaZa");
        assert!(compartment.contains(&Item('k')));
        assert!(compartment.contains(&Item('K')));
        assert!(compartment.contains(&Item('a')));
        assert!(compartment.contains(&Item('A')));
        assert!(compartment.contains(&Item('L')));
        assert!(compartment.contains(&Item('O')));
        assert!(compartment.contains(&Item('l')));
        assert!(compartment.contains(&Item('Z')));
        assert!(compartment.contains(&Item('z')));
    }
}
