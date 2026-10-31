# ninbot-wrapper

Runs [Ninjabrain Bot](https://github.com/Ninjabrain1/Ninjabrain-Bot) as a child process and renders its stronghold triangulation, boat state and warnings as a single F3-style line in a PNG. The same image can be shown by waywall and by an OBS image source, so the overlay looks identical in game and on stream. The bot and the wrapper start and stop together: when the bot exits, the wrapper exits too.

The line is only drawn while the player is in a world and unpaused; otherwise the PNG is a 1×1 transparent image.

## Install

```sh
mise use -g github:afrigon/ninbot-wrapper
```

A Java runtime must be on `PATH`. Ninjabrain Bot is pinned to version 1.5.2, because the speedrun rules only whitelist specific versions (A.11.7.a); it is downloaded from GitHub on first start and cached in the user cache directory. The bot's API must be enabled in its settings.

## Usage

```sh
ninbot-wrapper "$XDG_RUNTIME_DIR/ninbot-wrapper/overlay.png" ~/.local/share/PrismLauncher/instances/mcsr/minecraft/wpstateout.txt
```

The first argument is the PNG to render to, the second is the game state file written by the State Output mod. Set `RUST_LOG=debug` for more detailed logs.

### waywall

waywall cannot reload an image object, so the configuration reads the PNG periodically and replaces the image when its bytes change:

```lua
local waywall = require("waywall")

local output = os.getenv("XDG_RUNTIME_DIR") .. "/ninbot-wrapper/overlay.png"
local state = os.getenv("HOME") .. "/.local/share/PrismLauncher/instances/mcsr/minecraft/wpstateout.txt"

waywall.listen("load", function()
    waywall.exec("ninbot-wrapper " .. output .. " " .. state)
end)
```

### OBS

Add an Image source pointing at the PNG. OBS reloads it when the file changes.

## Development

```sh
mise run build
mise run test
mise run lint
mise run format
```
