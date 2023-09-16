use crate::Rule;
use pest::error::Error;

pub fn format_parse_error(err: Error<Rule>) -> Error<Rule> {
    err.renamed_rules(|rule| match *rule {
        Rule::EOI => "EOF".to_owned(),
        Rule::file => "import, struct, enum, fn, or EOF".to_owned(),

        // Include other rules above that aren't straightforward when hitting parsing errors!
        other => format!("{:?}", other),
    })
}
