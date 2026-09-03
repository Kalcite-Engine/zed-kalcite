use zed_extension_api as zed;

struct KalciteExtension;

impl zed::Extension for KalciteExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let command = worktree.which("kalcite-lsp").ok_or_else(|| {
            "Kalcite LSP was not found on PATH. Install the Kallyup developer or full profile."
                .to_owned()
        })?;
        Ok(zed::Command {
            command,
            args: Vec::new(),
            env: Vec::new(),
        })
    }
}

zed::register_extension!(KalciteExtension);
