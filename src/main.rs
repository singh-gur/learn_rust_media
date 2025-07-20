#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    fn description(&self) -> String {
        match self {
            Media::Book { title, author } => {
                format!("Book, Title: {}, Author: {}", title, author)
            }
            Media::Movie { title, director } => {
                format!("Movie Title: {}, Director: {}", title, director)
            }
            Media::Audiobook { title } => {
                format!("Audiobook Title: {}", title)
            }
            Media::Podcast(id) => {
                format!("Podcast ID: {}", id)
            }
            Media::Placeholder => {
                format!("Placeholder")
            }
        }
    }
}

#[derive(Debug)]
struct Catalog {
    items: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { items: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.items.push(item);
    }

    fn get_by_index(&self, index: usize) -> Option<&Media> {
        if index >= self.items.len() {
            None
        } else {
            Some(&self.items[index])
        }
    }

    fn display(&self) {
        for item in &self.items {
            println!("{}", item.description());
        }
    }
}

fn main() {
    let book1 = Media::Book {
        title: String::from("Don Quixote"),
        author: String::from("Miguel de Cervantes"),
    };
    let movie = Media::Movie {
        title: String::from("The Dark Knight"),
        director: String::from("Christopher Nolan"),
    };
    let audiobook = Media::Audiobook {
        title: String::from("Moby Dick"),
    };

    let placeholder = Media::Placeholder;
    let podcast = Media::Podcast(12345);
    // println!("{}", book1.description());
    // println!("{}", movie.description());
    // println!("{}", audiobook.description());
    // print_media(&book1);
    // print_media(&movie);
    // print_media(&audiobook);

    let mut catalog = Catalog::new();

    catalog.add(book1);
    catalog.add(movie);
    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    // let value = catalog.get_by_index(45).unwrap_or(&Media::Placeholder);
    if let Some(value) = catalog.get_by_index(45) {
        println!("{:#?}", value.description());
    } else {
        println!("Index out of bounds");
    }

    catalog.display();
}
