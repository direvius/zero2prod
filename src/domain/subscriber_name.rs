#[derive(Debug)]
pub struct SubscriberName(String);

const NAME_MAX_LEN: usize = 256;

impl SubscriberName {
    pub fn parse(input: String) -> Result<SubscriberName, String> {
        if input.len() > NAME_MAX_LEN {
            return Err(format!("name length is greater then {}", NAME_MAX_LEN));
        }
        Ok(SubscriberName(input))
    }
}

impl AsRef<str> for SubscriberName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}


impl TryFrom<String> for SubscriberName {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SubscriberName::parse(value)
    }
}
