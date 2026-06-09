#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlossaryEntry {
    pub id: String,
    pub term: String,
    pub plain_text: String,
    pub professional_text: Option<String>,
    pub related_terms: Vec<String>,
}
