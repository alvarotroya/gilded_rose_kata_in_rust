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

pub struct GildedRose {
    pub items: Vec<Item>,
}

impl GildedRose {
    pub fn new(items: Vec<Item>) -> GildedRose {
        GildedRose { items }
    }

    pub fn update_quality(&mut self) {
        for i in 0..self.items.len() {
            if self.items[i].name == "Sulfuras, Hand of Ragnaros" {
                continue;
            }

            if self.items[i].name != "Aged Brie"
                && self.items[i].name != "Backstage passes to a TAFKAL80ETC concert"
            {
                if self.items[i].quality > 0 {
                    self.items[i].quality -= 1;
                }
                self.items[i].sell_in -= 1;
                if self.items[i].sell_in < 0 {
                    self.items[i].quality -= 1;
                }
                continue;
            }

            if self.items[i].name == "Aged Brie" {
                if self.items[i].quality < 50 {
                    self.items[i].quality += 1;
                }
                self.items[i].sell_in -= 1;

                if self.items[i].sell_in < 0 {
                    if self.items[i].quality < 50 {
                        self.items[i].quality += 1;
                    }
                }
            }

            if self.items[i].name == "Backstage passes to a TAFKAL80ETC concert" {
                if self.items[i].quality < 50 {
                    self.items[i].quality += 1;
                }
                if self.items[i].sell_in < 11 {
                    if self.items[i].quality < 50 {
                        self.items[i].quality += 1;
                    }
                }
                if self.items[i].sell_in < 6 {
                    if self.items[i].quality < 50 {
                        self.items[i].quality += 1;
                    }
                }
                self.items[i].sell_in -= 1;

                if self.items[i].sell_in < 0 {
                    self.items[i].quality = 0;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{GildedRose, Item};
    #[test]
    pub fn quality_and_sell_date_decrease_by_one() {
        let item = Item::new("Random Item", 1, 1);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_item = Item::new("Random Item", 0, 0);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
    pub fn quality_is_never_negative() {
        let item = Item::new("Random Item", 1, 0);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_item = Item::new("Random Item", 0, 0);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
    pub fn quality_degrades_twice_as_fast_after_sell_date() {
        let item = Item::new("Random Item", 1, 10);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();

        let expected_item = Item::new("Random Item", -1, 7);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
    pub fn aged_brie_increases_in_quality() {
        let item = Item::new("Aged Brie", 3, 1);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();

        let expected_item = Item::new("Aged Brie", 2, 2);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
    pub fn aged_brie_increases_quality_twice_as_fast_after_sell_date() {
        let item = Item::new("Aged Brie", 1, 0);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();
        rose.update_quality();

        let expected_item = Item::new("Aged Brie", -2, 5);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
    pub fn sulfuras_never_degrades_or_has_to_be_sold() {
        let item = Item::new("Sulfuras, Hand of Ragnaros", 0, 80);
        let items = vec![item];
        let mut rose = GildedRose::new(items);

        rose.update_quality();
        rose.update_quality();
        rose.update_quality();

        let expected_item = Item::new("Sulfuras, Hand of Ragnaros", 0, 80);
        assert_eq!(expected_item, rose.items[0]);
    }

    #[test]
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
}
