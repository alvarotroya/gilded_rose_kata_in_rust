use core::cmp::{max, min};
use std::fmt::{self, Display};

#[derive(PartialEq, Debug)]
pub struct Item {
    pub name: String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub fn new(name: impl Into<String>, sell_in: i32, quality: i32) -> Item {
        Item {
            name: name.into(),
            sell_in,
            quality,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}, {}", self.name, self.sell_in, self.quality)
    }
}

#[derive(Debug, PartialEq)]
enum ItemType {
    RegularItem,
    AgedBrie,
    BackstagePasses,
    Sulfuras,
    ConjuredItem,
}

fn get_item_type(item: &Item) -> ItemType {
    if item.name.starts_with("Aged Brie") {
        ItemType::AgedBrie
    } else if item.name.starts_with("Backstage passes") {
        ItemType::BackstagePasses
    } else if item.name.starts_with("Sulfuras") {
        ItemType::Sulfuras
    } else if item.name.starts_with("Conjured") {
        ItemType::ConjuredItem
    } else {
        ItemType::RegularItem
    }
}

fn update_regular_item(item: &mut Item) {
    if item.sell_in > 0 {
        item.quality -= 1;
    } else {
        item.quality -= 2;
    }

    item.quality = max(item.quality, 0);
    item.sell_in -= 1;
}

fn update_conjured_item(item: &mut Item) {
    item.quality -= 2;

    item.quality = max(item.quality, 0);
    item.sell_in -= 1;
}

fn update_aged_brie(item: &mut Item) {
    if item.sell_in > 0 {
        item.quality += 1;
    } else {
        item.quality += 2;
    }

    item.quality = min(item.quality, 50);
    item.sell_in -= 1;
}

fn update_backstage_passes(item: &mut Item) {
    if item.sell_in <= 0 {
        item.quality = 0;
    } else if item.sell_in < 6 {
        item.quality += 3;
    } else if item.sell_in < 11 {
        item.quality += 2;
    } else {
        item.quality += 1;
    }

    item.quality = min(item.quality, 50);
    item.sell_in -= 1;
}

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for item in self.items.iter_mut() {
            match get_item_type(item) {
                ItemType::Sulfuras => {}
                ItemType::AgedBrie => update_aged_brie(item),
                ItemType::BackstagePasses => update_backstage_passes(item),
                ItemType::ConjuredItem => update_conjured_item(item),
                ItemType::RegularItem => update_regular_item(item),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_item_type, GildedRose, Item, ItemType};
    #[test]
    pub fn quality_and_sell_date_decrease_by_one() {
        let items = vec![Item::new("Random Item", 1, 1)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![Item::new("Random Item", 0, 0)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn quality_is_never_negative() {
        let items = vec![Item::new("Random Item", 1, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![Item::new("Random Item", 0, 0)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn quality_degrades_twice_as_fast_after_sell_date() {
        let items = vec![Item::new("Random Item", 1, 10)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();

        let expected_items = vec![Item::new("Random Item", -1, 7)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn aged_brie_increases_in_quality() {
        let items = vec![Item::new("Aged Brie", 3, 1)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![Item::new("Aged Brie", 2, 2)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn aged_brie_increases_quality_twice_as_fast_after_sell_date() {
        let items = vec![Item::new("Aged Brie", 1, 0)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();
        rose.update_quality();

        let expected_items = vec![Item::new("Aged Brie", -2, 5)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn sulfuras_never_degrades_or_has_to_be_sold() {
        let items = vec![Item::new("Sulfuras, Hand of Ragnaros", 0, 80)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();
        rose.update_quality();

        let expected_items = vec![Item::new("Sulfuras, Hand of Ragnaros", 0, 80)];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    // TODO: split up this test and modularize the code
    pub fn backstage_passes_increase_quality_relative_to_sell_date() {
        let items = vec![
            Item::new("Backstage passes to a TAFKAL80ETC concert", 11, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 9, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 5, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 4, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 1, 3),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 3),
        ];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![
            Item::new("Backstage passes to a TAFKAL80ETC concert", 10, 4),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 9, 5),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 8, 5),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 4, 6),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 3, 6),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 6),
            Item::new("Backstage passes to a TAFKAL80ETC concert", -1, 0),
        ];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn quality_is_always_lower_than_50() {
        let items = vec![
            Item::new("Aged Brie", 0, 50),
            Item::new("Aged Brie", -1, 49),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 1, 50),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 1, 48),
        ];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![
            Item::new("Aged Brie", -1, 50),
            Item::new("Aged Brie", -2, 50),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 50),
            Item::new("Backstage passes to a TAFKAL80ETC concert", 0, 50),
        ];
        assert_eq!(expected_items, rose.items);
    }

    #[test]
    pub fn quality_of_conjured_items_decreases_twice_as_fast() {
        let items = vec![Item::new("Conjured Mana Cake", 1, 2)];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_items = vec![Item::new("Conjured Mana Cake", 0, 0)];
        assert_eq!(expected_items, rose.items);
    }

    macro_rules! test_get_item_type {
        ($($name:ident: $item_type:expr, [$($value:expr),*],)*) => {
            $(
                #[test]
                fn $name() {
                    // Iterate over the slice of item names
                    for item_name in &[$($value),*] {
                        let item = Item::new((*item_name).to_string(), 0, 0);
                        let item_type = get_item_type(&item);

                        // Assert the item type is as specified for each item name
                        assert_eq!(item_type, $item_type);
                    }
                }
            )*
        };
    }

    test_get_item_type! {
        test_regular_items: ItemType::RegularItem, ["Random Item Name", "A regular item", "A conjured item"],
        test_conjured_items: ItemType::ConjuredItem, ["Conjured Item 1", "Conjured something else"],
        test_aged_brie_items: ItemType::AgedBrie, ["Aged Brie item", "Aged Brie something else"],
        test_backstage_passes_items: ItemType::BackstagePasses, ["Backstage passes to something", "Backstage passes to something else"],
        test_sulfura_items: ItemType::Sulfuras, ["Sulfuras Something"],
    }
}
