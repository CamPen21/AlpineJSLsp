// use tree_sitter::Parser;
// Note that  we must have our logging only write out to stderr.

// let mut parser = Parser::new();
// parser.set_language(tree_sitter_html::language()).expect("Error loading html grammar");
// let source_code = "<div>a</div>";
// let tree = parser.parse(source_code, None).unwrap();
// let root_node = tree.root_node();
// Create the transport. Includes the stdio (stdin and stdout) versions but this could
// also be implemented to use sockets or HTTP.

use lsp_server::{Request, RequestId, ExtractError};
use lsp_types::{ServerCapabilities, OneOf, CompletionOptions};
use serde_json::Value;

pub fn get_server_capabilities() -> Value {
    let chars = vec!("x".to_owned());
    serde_json::to_value(&ServerCapabilities {
        definition_provider: Some(OneOf::Left(true)),
        completion_provider: Some(CompletionOptions {
            trigger_characters: Some(chars),
            ..Default::default()
        }),
        ..Default::default()
    }).unwrap()
}

// Gets the requestId and parameters for a specific request of type R
pub fn cast<R>(req: Request) -> Result<(RequestId, R::Params), ExtractError<Request>>
where
    R: lsp_types::request::Request,
    R::Params: serde::de::DeserializeOwned,
{
    req.extract(R::METHOD)
}
