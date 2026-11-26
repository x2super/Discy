<p align="center">
  <img src="packaging/icon.svg" width="96" height="96" alt="Discy logo">
</p>

<h1 align="center">Discy</h1>

<p align="center">
  <a href="https://github.com/x2super/Discy/releases/latest"><img src="https://img.shields.io/github/v/release/x2super/Discy?label=release&color=5865F2" alt="Latest release"></a>
  <a href="#get-started"><img src="https://img.shields.io/badge/Windows%20%7C%20Linux-0078D4" alt="Windows and Linux"></a>
  <a href="LICENSE"><img src="https://img.shields.io/github/license/x2super/Discy?color=blue" alt="License: GPL 3.0"></a>
</p>

<p align="center">
  <b>A fast, lightweight native Discord client without the web browser.</b><br>
  About 30 MB of RAM idle, where Discord's Electron app takes ~720 MB (<a href="#measured-against-discord">measured as Vesktop</a>).
</p>

<p align="center">
  Native app written in Rust · Voice, video and screen sharing · Create &amp; join servers in-app
</p>

<p align="center">
  <a href="#windows-10-and-11"><b>Download for Windows</b></a>
  &nbsp;·&nbsp;
  <a href="#linux"><b>Download for Linux</b></a>
  &nbsp;·&nbsp;
  <a href="https://github.com/x2super/Discy/releases"><b>Releases</b></a>
</p>

<p align="center">
  <img src="docs/images/yapfast.png" alt="Discy showing a group DM, with the server list on the left, the conversation in the middle and its members on the right"><br>
  <sub>A test account on a fake Discord server.</sub>
</p>

## Why Discy?

Discord's desktop app runs a full Chromium browser to display Discord's website,
consuming around 720 MB of RAM while idle.

**Discy** is a native Rust desktop client (forked from [Yapfast](https://github.com/yapfast/yapfast))
that talks directly to Discord's API and draws its own interface with
[iced](https://iced.rs) — no browser engine anywhere. Your servers, DMs,
friends, voice channels, screen sharing, and GIFs are all there, starting in
a fraction of a second and using ~30 MB of RAM idle.

### What's customized in Discy

- **Discy branding & icon**: Native `discy.exe` executable, Windows Setup Installer (`Discy-…-windows-x64-setup.exe`), system tray, and notifications.
- **Create & Join Server flow**: Clicking the `+` button in the server rail opens an **Add a Server** dialog letting you choose between **Create a Server** (`POST /guilds` in-app) and **Join a Server** (direct in-app `POST /invites/{code}` with no separate "Find Invite" step).

## Measured against Discord

<table>
  <tr><th></th><th>Vesktop memory</th><th>Discy memory</th><th>Vesktop CPU</th><th>Discy CPU</th></tr>
  <tr><td>Idle</td><td>724 MB</td><td>30 MB (−96%)</td><td>0.11%</td><td>0.015% (−86%)</td></tr>
  <tr><td>Image-heavy channel</td><td>706 MB</td><td>41 MB (−94%)</td><td>0.17%</td><td>0.008% (−95%)</td></tr>
  <tr><td>In a call</td><td>888 MB</td><td>44 MB (−95%)</td><td>0.46%</td><td>0.33% (−28%)</td></tr>
  <tr><td>Sharing your screen¹</td><td>1,013 MB</td><td>113 MB (−89%)</td><td>2.15%</td><td>1.1% (−48%)</td></tr>
  <tr><th></th><th colspan="2">Discord</th><th colspan="2">Discy</th></tr>
  <tr><td>Opening the app²</td><td colspan="2" align="center">8 s</td><td colspan="2" align="center">0.22 s (36× faster)</td></tr>
</table>

<details>
<summary><b>About these numbers</b></summary>

<br>

Measurements are from upstream Yapfast against Vesktop (Electron + Vencord) and
Discord's desktop client. CPU counts the whole processor as 100%.

¹ Sharing a 1920×1080 monitor at 30 FPS while in a call, using hardware H.264
encoding on an NVIDIA RTX 3090 (software x264 uses ~2.5%).

² Cold launch from clicking the icon to reading messages, showing cached
history while catching up live.

Methodology and hardware details are documented in
[docs/measurements/README.md](docs/measurements/README.md).

</details>

## Features

### Favorites

Discord's saved Favorites appear behind the star in the server rail, with their
categories and order. Right-click a channel, DM or group DM to add or remove it;
changes sync with your Discord account. See [Favorites](docs/discord/favorites.md).

### The GIF picker

Your favorite GIFs live on disk as compact clips so the picker opens instantly.

<img src="docs/images/features/gif-picker.webp" alt="The GIF picker opening on its favorites over #general, and one of them sent to the channel.">

### Voice, cameras and Go Live

Voice channels and DM calls, with mute, deafen, per-user volume, noise
suppression (RNNoise & DeepFilterNet), echo removal, tunable voice gate, and
global push-to-talk / mute / deafen keybinds ([keybinds](docs/keybinds.md)).
Supports Go Live screen sharing, watching streams, cameras, and soundboards.

<img src="docs/images/features/voice.webp" alt="A voice channel of seven people, two with their cameras on. One person's stream opens large, then the grid comes back.">

### Forums and polls

Browse forum posts as cards with tags, top reactions and thumbnails, post and
reply beside the forum, and create or vote in polls.

<img src="docs/images/features/forum.webp" alt="A help forum's posts as cards; one opens beside the list with its replies, then closes.">

<p align="center">
  <img src="docs/images/features/poll.webp" width="640" alt="A movie night poll: an answer is picked and voted for, and the results fill in.">
</p>

### Themes

Ten built-in themes (including Discy, Bonfire, Catppuccin, and Tokyo Night)
plus a live theme creator and package support.

<img src="docs/images/features/themes.webp" alt="The same channel in each of the ten built-in themes, one after another.">

### Games on your profile

Automatic game detection on Linux, Windows and macOS (including Steam and Wine),
Rich Presence with details, timestamps, images and buttons, plus "Listening to
Spotify" integration. Configure in **Settings → Game activity**. See
[game detection](docs/discord/people.md#game-detection),
[Rich Presence](docs/discord/people.md#rich-presence), and
[Spotify](docs/discord/people.md#listening-to-spotify).

<p align="center">
  <img src="docs/images/features/playing.webp" width="260" alt="A profile card saying Playing Deadlock, with the game's icon.">
</p>

### Running a server

Full Server Settings (roles, emoji, stickers, bans, audit log, moderation) and
channel permissions. Create and edit channels and categories, invite people,
assign roles, change nicknames, time out, kick and ban, and manage members in
voice. Click `+` at the bottom of the server rail to **Create a Server** or
**Join a Server** directly in-app by invite link or code, or open clicked
invite and scheduled event links in-app.

<p align="center">
  <img src="docs/images/features/server-settings.webp" width="640" alt="Server Settings on a role's Display page: its name, a gold color picked, and its icon.">
</p>

### Everything else

- **Servers, channels, DMs and group DMs**, all live, with member lists,
  message requests, and server folders.
- **Messages** with Markdown, embeds, custom emoji, stickers, spoilers,
  replies, edits, reactions, pins, forwarding, attachments, voice messages,
  bot components/modals, and slash/context commands.
- **Threads**, **full server search**, **rich user profiles**, **friends**,
  **multi-account switching**, **desktop notifications & inbox**, and **UI sound packs**.

### Limitations

- Activities are not supported yet.
- Creating and editing server folders is not supported yet.
- New-member verification forms (`show_verification_form`) open in Discord's
  web app to finish.
- If Discord requests a captcha while signing in, creating, or joining a
  server, Discy opens the solver page in your browser and continues once solved.

## Account Safety Notice

> [!WARNING]
> Third-party clients, Discy included, are against Discord's Terms of Service.
> Discy only performs actions you explicitly click and does not run automation
> or spam, but using any third-party client carries some risk to your account.
> Use at your own discretion.

## Privacy

- Connects directly from your computer to Discord (or your Matrix homeserver
  when built with `--features matrix`) with no intermediary server.
- Supports QR code login so Discy never sees your password; tokens are stored
  in your OS credential store.
- No analytics or telemetry.

## Get started

### Windows 10 and 11

1. Download `Discy-…-windows-x64-setup.exe` from the
   [latest release](https://github.com/x2super/Discy/releases/latest).
2. Run the setup installer. It installs per-user under
   `%LOCALAPPDATA%\Programs\Discy` with no administrator prompt. Because the
   installer is unsigned, Windows SmartScreen may show a warning: click
   **More info** → **Run anyway**.
3. Sign in by scanning the QR code with Discord on your phone (**Settings →
   Scan QR Code**) or with a token.

Prefer a portable build? Download `Discy-…-windows-x64.zip`, extract it, and
run `discy.exe`.

Discy can optionally start with Windows in the system tray (configurable in
the installer or **Settings → Advanced**).

### Linux

Build from source with Cargo or Nix (see [Build it yourself](#build-it-yourself)).

## Build it yourself

### Windows (Native Cargo + Inno Setup Installer)

Install Visual Studio Build Tools (C++ & Windows SDK), LLVM/Clang
(`clang-cl` and `libclang`), NASM, and optionally [Inno Setup 6](https://jrsoftware.org/isinfo.php)
for compiling the Windows installer. See [docs/building.md#windows](docs/building.md#windows).

From Developer PowerShell for VS:

```powershell
# Development / fast build and tests
cargo build --locked --profile fast
cargo run --locked --profile fast -- --preview
cargo test --locked --profile fast --workspace

# Optimized release build (produces target\release\discy.exe)
$env:YAPFAST_RELEASE = "1"
cargo build --locked --release
```

### Linux (Nix or Cargo)

```sh
# Preview UI without an account
nix develop path:./nix --command cargo run --locked --profile fast -- --preview

# Run Discord release flavour
nix develop path:./nix --command cargo run --locked --profile fast

# Run with Matrix support too
nix develop path:./nix --command cargo run --locked --profile fast --features matrix

# Full checks
scripts/check --fast
```

| Read this | For |
| --- | --- |
| [Contributing](CONTRIBUTING.md) | Checks, memory targets, commit style |
| [Development](docs/development.md) | Preview mode, local test servers, voice tests |
| [Architecture](docs/architecture.md) | How `yapfast-core` and the iced frontend fit together |
| [Building](docs/building.md) | Toolchain, Cargo features, checks, releases |
| [Discord](docs/discord/README.md) | Discord protocol notes and fake server tests |
| [Voice](docs/voice.md) | Calls, devices and audio processing |
| [Windows](docs/windows-beta.md) | Packaging, installer and Windows checks |
| [Measurements](docs/measurements/README.md) | Memory and CPU benchmarks |

## License

Discy is free software under the [GNU General Public License](LICENSE),
version 3 or later (`GPL-3.0-or-later`). Builds with Matrix also include
MatrixRTC libraries under the AGPL.

## Credits & Upstream

Discy is built on [iced](https://iced.rs), [LiveKit](https://livekit.io) / WebRTC,
[DeepFilterNet](https://github.com/Rikorose/DeepFilterNet), and Discord API
documentation from [Discord Userdoccers](https://docs.discord.food).

---

<p align="center">
  <sub>
    <b>Forked from <a href="https://github.com/yapfast/yapfast">Yapfast</a></b> (<a href="https://github.com/yapfast/yapfast">github.com/yapfast/yapfast</a>), licensed under GPL-3.0-or-later.<br>
    Maintained at <a href="https://github.com/x2super/Discy">github.com/x2super/Discy</a> by <a href="https://github.com/x2super">x2super</a>.<br>
    Discy is not affiliated with or endorsed by Discord Inc. Discord is a trademark of Discord Inc.
  </sub>
</p>
