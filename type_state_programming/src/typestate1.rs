type word =  &'static str;

#[derive(Debug)]
pub struct Rawtext(&'static str);
#[derive(Debug)]
pub struct Parsedtext(Vec<word>);
#[derive(Debug)]
pub struct  formattedText(String);

impl Rawtext{
    pub fn new(raw: &'static str) -> Self{
        Self(raw)
    }
}

impl Rawtext {
    pub fn parse(self) -> Parsedtext {
        let parsed =  self.0.split(' ').collect();
        Parsedtext(parsed)
    }
}
impl Parsedtext {
    pub fn format(self) -> formattedText {
        formattedText(self.0.join("."))
    }
}
