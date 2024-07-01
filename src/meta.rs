use atom_syndication::Person;
use atom_syndication::Feed;

pub struct FeedMetadata {
    pub author: Person,
    complete: bool,
}

impl Default for FeedMetadata {
    fn default() -> FeedMetadata {
        FeedMetadata{
            author: Person{
                name: "No One".to_string(),
                email: Some("none@devnull.com".to_string()),
                uri: Some("https://devnull.com".to_string()),
            },
            complete: false,
        }
    }
}

impl FeedMetadata {
    pub fn apply(&self, feed: &mut Feed) {
        let mut persons = Vec::<Person>::new();
        persons.push(self.author.clone());
        feed.set_authors(persons);
    }
}
