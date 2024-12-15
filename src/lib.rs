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
            command: "/usr/bin/nelua".to_string(),
            args: vec![
                "-L".to_string(),
                "/home/kome/Documents/code-projects/random/nelua-lsp/".to_string(),
                "--script".to_string(),
                "/home/kome/Documents/code-projects/random/nelua-lsp/main.lua".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(NeLuaExtension);
