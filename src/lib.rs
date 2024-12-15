use zed::LanguageServerId;
use zed_extension_api::{self as zed};

struct NeLuaExtension {}

impl zed::Extension for NeLuaExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command, String> {
        Ok(zed::Command {
            command: "/path/to/nelua".to_string(), // change
            args: vec![
                "-L".to_string(),
                "/path/to/nelua-lsp/".to_string(), // change
                "--script".to_string(),
                "/path/to/nelua-lsp/main.lua".to_string(), // change
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NeLuaExtension);
