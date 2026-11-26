//! Adding a Discord server from the "+" at the foot of the server rail
//! (choosing Create a Server or Join a Server), or joining by a clicked
//! invite link. Joining looks up the invite and joins in-app (`POST
//! /invites/{code}`); creating posts to `POST /guilds`. Both solve any
//! captcha Discord asks for in the browser.

use super::*;
use crate::chat::{InvitePreview, JoinedServer};

/// Which page the Add a Server dialog shows.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JoinStep {
    #[default]
    Choose,
    Create,
    Join,
}

/// The Add a Server / Join a Server dialog, while it's open.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JoinDialog {
    /// Which page is open: Choose, Create, or Join.
    pub step: JoinStep,
    /// The code of the invite looked up or shown.
    pub code: Option<String>,
    /// What the invite shows, once Discord answered.
    pub preview: Option<InvitePreview>,
    /// This account is in the server already, so the dialog opens it.
    pub member: bool,
    /// A clicked invite link opened the dialog, which then has no field.
    pub from_link: bool,
    /// Direct join from the Join a Server input: join as soon as the
    /// invite resolves, without stopping at an intermediate preview card.
    pub auto_join: bool,
    pub resolving: bool,
    pub joining: bool,
    pub creating: bool,
    /// The page with the captcha Discord asked for before joining or creating.
    pub captcha: Option<String>,
    /// The server, joined, asks new members to fill in its form first,
    /// which Discy leaves to Discord's web app at this address.
    pub form: Option<String>,
    pub error: Option<String>,
}

impl Application {
    /// Opens Add a Server on the choice step; nothing without a linked
    /// Discord account (or preview).
    pub(super) fn open_join(&mut self) -> Vec<Effect> {
        if self.state.discord_online || self.state.preview {
            self.state.join = Some(JoinDialog {
                step: JoinStep::Choose,
                ..JoinDialog::default()
            });
        }
        vec![]
    }

    /// Switches between Choose, Create, and Join while the dialog is open
    /// and idle, clearing any previous attempt's preview, captcha or error.
    pub(super) fn set_join_step(&mut self, step: JoinStep) -> Vec<Effect> {
        let Some(join) = self.state.join.as_mut().filter(|join| {
            !join.resolving
                && !join.joining
                && !join.creating
                && !join.from_link
                && join.form.is_none()
        }) else {
            return vec![];
        };
        *join = JoinDialog {
            step,
            ..JoinDialog::default()
        };
        vec![]
    }

    /// Resolves and immediately joins an invite typed or pasted into Join a
    /// Server, with no intermediate Find step.
    pub(super) fn join_invite(&mut self, typed: String) -> Vec<Effect> {
        let Some(join) = self.state.join.as_mut().filter(|join| {
            !join.resolving && !join.joining && !join.creating && join.form.is_none()
        }) else {
            return vec![];
        };
        let Some(code) = typed_code(&typed) else {
            join.error = Some("Paste an invite link, such as https://discord.gg/discy.".into());
            return vec![];
        };
        *join = JoinDialog {
            step: JoinStep::Join,
            code: Some(code.clone()),
            auto_join: true,
            resolving: true,
            ..JoinDialog::default()
        };
        self.resolve(code, Some(typed))
    }

    /// Opens the dialog for a clicked invite link, by its code, and looks
    /// the invite up.
    pub(super) fn open_invite_link(&mut self, code: String) -> Vec<Effect> {
        if !self.state.discord_online && !self.state.preview {
            return vec![];
        }
        self.state.join = Some(JoinDialog {
            step: JoinStep::Join,
            code: Some(code.clone()),
            from_link: true,
            resolving: true,
            ..JoinDialog::default()
        });
        self.resolve(code, None)
    }

    fn resolve(&self, code: String, typed: Option<String>) -> Vec<Effect> {
        if self.state.preview {
            return vec![];
        }
        vec![Effect::Backend(Command::Discord(
            DiscordCommand::ResolveInvite { code, typed },
        ))]
    }

    pub(super) fn invite_resolved(
        &mut self,
        code: String,
        result: Result<InvitePreview, String>,
    ) -> Vec<Effect> {
        let Some(join) = self
            .state
            .join
            .as_mut()
            .filter(|join| join.resolving && join.code.as_ref() == Some(&code))
        else {
            return vec![];
        };
        join.resolving = false;
        let preview = match result {
            Ok(preview) => preview,
            Err(error) => {
                join.auto_join = false;
                join.error = Some(error);
                return vec![];
            }
        };
        let member = self
            .state
            .servers
            .iter()
            .any(|server| server.id == preview.server);
        join.member = member;
        // A clicked link or direct join to a server this account is already
        // in goes straight there.
        if member && (join.from_link || join.auto_join) {
            return self.open_joined(&preview.server, preview.room);
        }
        let auto_join = std::mem::take(&mut join.auto_join);
        join.preview = Some(preview);
        if auto_join {
            return self.join_server();
        }
        vec![]
    }

    /// Joins the server in-app (`POST /invites/{code}`), or opens it if the
    /// account is already a member.
    pub(super) fn join_server(&mut self) -> Vec<Effect> {
        let Some(join) = self
            .state
            .join
            .as_mut()
            .filter(|join| !join.joining && !join.creating && join.form.is_none())
        else {
            return vec![];
        };
        let Some(preview) = join.preview.clone() else {
            return vec![];
        };
        if join.member {
            return self.open_joined(&preview.server, preview.room);
        }
        if self.state.preview {
            return vec![];
        }
        join.joining = true;
        join.error = None;
        join.captcha = None;
        vec![Effect::Backend(Command::Discord(
            DiscordCommand::JoinServer {
                preview: Box::new(preview),
                from_link: join.from_link,
            },
        ))]
    }

    /// Creates a new Discord server with `name`.
    pub(super) fn create_server(&mut self, name: String) -> Vec<Effect> {
        let Some(join) = self.state.join.as_mut().filter(|join| {
            join.step == JoinStep::Create && !join.creating && !join.joining && !join.resolving
        }) else {
            return vec![];
        };
        let trimmed = name.trim();
        let chars = trimmed.chars().count();
        if !(2..=100).contains(&chars) {
            join.error = Some("Server name must be between 2 and 100 characters.".into());
            return vec![];
        }
        if self.state.preview {
            join.error = Some("Creating a server requires a linked Discord account.".into());
            return vec![];
        }
        join.creating = true;
        join.error = None;
        join.captcha = None;
        vec![Effect::Backend(Command::Discord(
            DiscordCommand::CreateServer {
                name: trimmed.to_owned(),
            },
        ))]
    }

    pub(super) fn create_captcha(&mut self, page: String) -> Vec<Effect> {
        let Some(join) = self.state.join.as_mut().filter(|join| join.creating) else {
            return vec![];
        };
        join.captcha = Some(page.clone());
        vec![Effect::OpenPage(page)]
    }

    pub(super) fn server_created(&mut self, result: Result<String, String>) -> Vec<Effect> {
        let Some(join) = self.state.join.as_mut().filter(|join| join.creating) else {
            return vec![];
        };
        join.creating = false;
        join.captcha = None;
        match result {
            Ok(server) => {
                self.state.join = None;
                if self.state.servers.iter().any(|listed| listed.id == server) {
                    return self.dispatch(Action::Navigate(Navigation::Server(server)));
                }
                self.pending_server = Some(server);
                vec![]
            }
            Err(error) => {
                join.error = Some(error);
                vec![]
            }
        }
    }

    pub(super) fn join_captcha(&mut self, code: String, page: String) -> Vec<Effect> {
        let Some(join) = self
            .state
            .join
            .as_mut()
            .filter(|join| join.joining && join.code.as_ref() == Some(&code))
        else {
            return vec![];
        };
        join.captcha = Some(page.clone());
        vec![Effect::OpenPage(page)]
    }

    /// Opens the captcha Discord asked for again, as linking's does.
    pub(super) fn open_join_captcha(&self) -> Vec<Effect> {
        match self
            .state
            .join
            .as_ref()
            .and_then(|join| join.captcha.clone())
        {
            Some(page) => vec![Effect::OpenPage(page)],
            None => vec![],
        }
    }

    pub(super) fn server_joined(
        &mut self,
        code: String,
        result: Result<JoinedServer, String>,
    ) -> Vec<Effect> {
        let Some(join) = self
            .state
            .join
            .as_mut()
            .filter(|join| join.joining && join.code.as_ref() == Some(&code))
        else {
            return vec![];
        };
        join.joining = false;
        join.captcha = None;
        let joined = match result {
            Ok(joined) => joined,
            Err(error) => {
                join.error = Some(error);
                return vec![];
            }
        };
        if joined.form {
            // The server lists once Discord sends it, and the dialog stays
            // to send the form to the web app.
            join.form = Some(match discord::parse_room(&joined.room) {
                Some((guild, channel)) => {
                    format!("https://discord.com/channels/{guild}/{channel}")
                }
                None => "https://discord.com/app".into(),
            });
            self.pending_open = Some(joined.room);
            return vec![];
        }
        self.open_joined(&joined.server, joined.room)
    }

    /// Opens the server's form in Discord's web app, and closes the dialog.
    pub(super) fn open_join_form(&mut self) -> Vec<Effect> {
        match self.state.join.take().and_then(|join| join.form) {
            Some(page) => vec![Effect::OpenPage(page)],
            None => vec![],
        }
    }

    /// Closes the dialog and opens the invite's channel, now or once it
    /// lists, or else its server.
    fn open_joined(&mut self, server: &str, room: String) -> Vec<Effect> {
        self.state.join = None;
        if self.state.rooms.iter().any(|listed| listed.id == room) {
            return self.dispatch(Action::Open(room));
        }
        if self.state.servers.iter().any(|listed| listed.id == server) {
            return self.dispatch(Action::Navigate(Navigation::Server(server.to_owned())));
        }
        self.pending_open = Some(room);
        vec![]
    }
}

/// The invite code in what someone typed, as Discord reads it.
fn typed_code(typed: &str) -> Option<String> {
    #[cfg(feature = "discord")]
    {
        yapfast_discord::invite::typed_code(typed).map(str::to_owned)
    }
    #[cfg(not(feature = "discord"))]
    {
        let _ = typed;
        None
    }
}
