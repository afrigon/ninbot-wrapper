# ninbot-wrapper

## Speedrun legality

Before adding or changing a feature, validate it against the Minecraft speedrun rules at
https://mc.sr/rules/rules_v8.pdf and state the result. A feature that is not clearly allowed is
raised before any code is written.

- The overlay shows only what an unmodified Ninjabrain Bot shows (A.11.7.a.i). Hiding data is fine;
  data the bot does not display, or interpretations it does not offer, are not.
- Mod output files such as `wpstateout.txt` are read only for the uses A.8.14.a allows.
- World and log files of the game are never read (A.3.10).
- Ninjabrain Bot stays on a version the rules whitelist (A.11.7.a).
