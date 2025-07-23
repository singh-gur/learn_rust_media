#[derive(Debug)]
pub enum Media {
    Book { title: String, author: String },
    Movie { title: String, director: String },
    Audiobook { title: String },
    Podcast(u32),
    Placeholder,
}

impl Media {
    pub fn description(&self) -> String {
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
