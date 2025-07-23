use super::media::Media;

#[derive(Debug)]
pub struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    pub fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    pub fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    pub fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index >= self.items.len() {
            None
        } else {
            Some(&self.items[index])
        }
    }

    pub fn display(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }
}
