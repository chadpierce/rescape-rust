//TODO implement color
#[derive(Clone, Debug)]
pub struct Messages {
    messages: Vec<(String, String)>,
}
impl Messages {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }
    // msg and text color as tuple
     pub fn add<T: Into<String>>(&mut self, message: T, color: T) {
         self.messages.push((message.into(), color.into()));
    }
    // Create a DoubleEndedIterator over the messages
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &(String, String)> {
         self.messages.iter()
    }
    // pub fn get_newest(&self) -> String {
    //     self.messages[0 as usize]
    // }
}