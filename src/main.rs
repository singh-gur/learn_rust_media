#[derive(Debug)]
enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
}

fn print_media(media: &Media) {
    println!("Media: {:#?}", media)
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
        }
    }
}

#[derive(Debug)]
struct Catalog {
    media: Vec<Media>,
}

impl Catalog {
    fn new() -> Self {
        Catalog { media: Vec::new() }
    }

    fn add(&mut self, item: Media) {
        self.media.push(item);
    }

    fn display(&self) {
        for item in &self.media {
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

    // println!("{}", book1.description());
    // println!("{}", movie.description());
    // println!("{}", audiobook.description());
    print_media(&book1);
    // print_media(&movie);
    // print_media(&audiobook);

    let mut catalog = Catalog::new();

    catalog.add(book1);
    catalog.add(movie);
    catalog.add(audiobook);

    catalog.display();
}
