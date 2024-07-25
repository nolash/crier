use atom_syndication::Person;
use atom_syndication::Feed;
use uuid::Uuid;

#[derive(Debug)]
pub enum Error {
    IncompleteFeedMetadata,
}


pub struct FeedMetadata {
    pub author: Person,
    pub title: String,
    pub id: String,
    incomplete: bool,
}

impl Default for FeedMetadata {
    fn default() -> FeedMetadata {
        FeedMetadata{
            author: Person{
                name: "?".to_string(),
                email: Some("?".to_string()),
                uri: Some("?".to_string()),
            },
            title: String::from("?"),
            id: Uuid::new_v4().to_string(),
            incomplete: true,
        }
    }
}

impl FeedMetadata {
    pub fn force(&mut self) {
        self.incomplete = false;
    }

    pub fn apply(&self, feed: &mut Feed) -> Result<(), Error> {
        if self.incomplete {
            return Err(Error::IncompleteFeedMetadata);
        }
        let mut persons = Vec::<Person>::new();
        persons.push(self.author.clone());
        feed.set_authors(persons);
        feed.set_title(self.title.clone());
        Ok(())
    }
}
