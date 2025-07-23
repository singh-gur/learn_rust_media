mod content;
use content::catalog::Catalog;
use content::media::Media;

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

    let mut catalog = Catalog::new();

    catalog.add(book1);
    catalog.add(movie);
    catalog.add(audiobook);
    catalog.add(podcast);
    catalog.add(placeholder);

    if let Some(value) = catalog.get_by_index(45) {
        println!("{:#?}", value.description());
    } else {
        println!("Index out of bounds");
    }

    catalog.display();
}
