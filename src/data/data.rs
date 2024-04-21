use serde_json::Value;
use std::collections::HashMap;

pub struct Categories {
    categories: Vec<HashMap<String, Value>>,
    error_response: HashMap<String, Value>,
}

pub struct CategoryDetails {
    details: HashMap<String, Value>,
}

impl Categories {
    pub fn new() -> Categories {
        let mut ctg = Categories {
            categories: Vec::new(),
            error_response: HashMap::new(),
        };

        ctg.load_data();
        log::info!("data.rs::new - Categories: {:?}", ctg.categories.len());
        ctg
    }

    pub fn add_category(&mut self, category: HashMap<String, Value>) {
        self.categories.push(category);
    }

    pub fn get_categories(&self, mut count: i32) -> Vec<HashMap<String, Value>> {
        if count < 0 {
            count = self.categories.len() as i32 - count.abs();
            if count < 0 {
                count = 0;
            }
        }
        if count > self.categories.len() as i32 {
            count = count.min(self.categories.len() as i32);
        }
        log::info!("data.rs::get_categories - count: {:?}", count);
        if self.categories.len() as i32 <= count && self.categories.len() as i32 > 0 {
            self.categories.to_vec()
        } else {
            self.categories[..count as usize].to_vec()
        }
    }

    pub fn get_category(&self, index: i32) -> &HashMap<String, Value> {
        log::info!("data.rs::get_category - index: {:?}", index);
        for category in self.categories.iter() {
            if category["id"] == index {
                return category;
            }
        }

        &self.error_response
    }

    fn load_data(&mut self) {
        let data = include_str!("../data/categories.json");
        let json: Vec<HashMap<String, Value>> = serde_json::from_str(data).unwrap();
        for category in json.iter() {
            self.add_category(category.clone());
        }
    }
}

impl CategoryDetails {
    pub fn new() -> CategoryDetails {
        let mut details = CategoryDetails {
            details: HashMap::new(),
        };

        details.load_data();
        details
    }

    pub fn get_details(&self) -> &HashMap<String, Value> {
        &self.details
    }

    pub fn get_detail(&self, category_number: &str) -> &Value {
        log::info!("data.rs::get_detail - category_number: {:?}", category_number);
        &self.details[category_number]
    }

    fn load_data(&mut self) {
        let data = include_str!("../data/category_details.json");
        let json: HashMap<String, Value> = serde_json::from_str(data).unwrap();
        self.details = json;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::TOTAL_CATEGORIES;

    #[test]
    fn test_categories() {
        let categories = Categories::new();
        assert_eq!(categories.get_categories(-16).len(), 0);
        assert_eq!(categories.get_categories(-15).len(), 0);
        assert_eq!(
            categories.get_categories(-14).len(),
            TOTAL_CATEGORIES as usize - 14
        );
        assert_eq!(
            categories.get_categories(-13).len(),
            TOTAL_CATEGORIES as usize - 13
        );
        assert_eq!(
            categories.get_categories(-12).len(),
            TOTAL_CATEGORIES as usize - 12
        );
        assert_eq!(
            categories.get_categories(-11).len(),
            TOTAL_CATEGORIES as usize - 11
        );
        assert_eq!(
            categories.get_categories(-10).len(),
            TOTAL_CATEGORIES as usize - 10
        );
        assert_eq!(
            categories.get_categories(-9).len(),
            TOTAL_CATEGORIES as usize - 9
        );
        assert_eq!(
            categories.get_categories(-8).len(),
            TOTAL_CATEGORIES as usize - 8
        );
        assert_eq!(
            categories.get_categories(-7).len(),
            TOTAL_CATEGORIES as usize - 7
        );
        assert_eq!(
            categories.get_categories(-6).len(),
            TOTAL_CATEGORIES as usize - 6
        );
        assert_eq!(
            categories.get_categories(-5).len(),
            TOTAL_CATEGORIES as usize - 5
        );
        assert_eq!(
            categories.get_categories(-4).len(),
            TOTAL_CATEGORIES as usize - 4
        );
        assert_eq!(
            categories.get_categories(-3).len(),
            TOTAL_CATEGORIES as usize - 3
        );
        assert_eq!(
            categories.get_categories(-2).len(),
            TOTAL_CATEGORIES as usize - 2
        );
        assert_eq!(
            categories.get_categories(-1).len(),
            TOTAL_CATEGORIES as usize - 1
        );
        assert_eq!(categories.get_categories(0).len(), 0);
        assert_eq!(categories.get_categories(1).len(), 1);
        assert_eq!(categories.get_categories(2).len(), 2);
        assert_eq!(categories.get_categories(3).len(), 3);
        assert_eq!(categories.get_categories(4).len(), 4);
        assert_eq!(categories.get_categories(5).len(), 5);
        assert_eq!(categories.get_categories(6).len(), 6);
        assert_eq!(categories.get_categories(7).len(), 7);
        assert_eq!(categories.get_categories(8).len(), 8);
        assert_eq!(categories.get_categories(9).len(), 9);
        assert_eq!(categories.get_categories(10).len(), 10);
        assert_eq!(categories.get_categories(11).len(), 11);
        assert_eq!(categories.get_categories(12).len(), 12);
        assert_eq!(categories.get_categories(13).len(), 13);
        assert_eq!(
            categories.get_categories(14).len(),
            TOTAL_CATEGORIES as usize
        );
        assert_eq!(
            categories.get_categories(15).len(),
            TOTAL_CATEGORIES as usize
        );
    }

    #[test]
    fn test_category() {
        let categories = Categories::new();
        let category = categories.get_category(2);
        assert_eq!(category["id"], 2);
        assert_eq!(category["title"], "baseball");
    }
}