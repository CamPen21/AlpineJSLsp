use std::error::Error;

use alpinejs_lsp::{cast, get_server_capabilities};
use lsp_types::{request::GotoDefinition, GotoDefinitionResponse, InitializeParams};
use lsp_types::{Location, Position, Range};

use lsp_server::{Connection, ExtractError, Message, Response};

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    let (connection, io_threads) = Connection::stdio();

    // Run the server and wait for the two threads to end (typically by trigger LSP Exit event).
    // The type of this is serde_json::Value. Which is a valid JSON type basically
    let server_capabilities = get_server_capabilities();
    let initialization_params = connection.initialize(server_capabilities)?;
    main_loop(connection, initialization_params)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(
    connection: Connection,
    params: serde_json::Value,
) -> Result<(), Box<dyn Error + Sync + Send>> {
    let _params: InitializeParams = serde_json::from_value(params).unwrap();
    for msg in &connection.receiver {
        match msg {
            Message::Request(req) => {
                if connection.handle_shutdown(&req)? {
                    return Ok(());
                }
                match cast::<GotoDefinition>(req) {
                    Ok((id, params)) => {
                        let url = params.text_document_position_params.text_document.uri;
                        eprintln!("Document url = {:?}", url);
                        let location = Location {
                            uri: url,
                            range: Range {
                                start: Position {
                                    line: 0,
                                    character: 0,
                                },
                                end: Position {
                                    line: 0,
                                    character: 0,
                                },
                            },
                        };
                        let mut locations = Vec::<Location>::new();
                        locations.push(location);
                        eprintln!("locations = {:?}", locations);
                        let result = Some(GotoDefinitionResponse::Array(locations));
                        let result = serde_json::to_value(&result).unwrap();
                        let resp = Response {
                            id,
                            result: Some(result),
                            error: None,
                        };
                        connection.sender.send(Message::Response(resp))?;
                        continue;
                    }
                    Err(err @ ExtractError::JsonError { .. }) => {
                        panic!("json err {err:?}");
                    }
                    Err(ExtractError::MethodMismatch(req)) => req,
                };
                // ...
            }
            Message::Response(resp) => {
                eprintln!("got response: {resp:?}");
            }
            Message::Notification(not) => {
                eprintln!("got notification: {not:?}");
            }
        }
    }
    Ok(())
}
