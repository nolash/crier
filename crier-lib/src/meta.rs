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
    flag: u8,
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
            flag: 0,
        }
    }
    
    
}

impl FeedMetadata {
    pub fn force(&mut self) {
        self.flag |= 3;
    }

    fn check_complete(&self) -> bool {
        self.flag >= 3
    }

    pub fn set_author(&mut self, author: Person) -> bool {
        self.author = author;
        self.flag |= 1;
        self.check_complete()
    }


    pub fn set_title(&mut self, title: String) -> bool {
        self.title = title;
        self.flag |= 2;
        self.check_complete()
    }

    pub fn apply(&self, feed: &mut Feed) -> Result<(), Error> {
        if !self.check_complete() {
            return Err(Error::IncompleteFeedMetadata);
        }
        let mut persons = Vec::<Person>::new();
        persons.push(self.author.clone());
        feed.set_authors(persons);
        feed.set_title(self.title.clone());
        Ok(())
    }
}
