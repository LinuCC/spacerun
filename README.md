# Idea

Emacs / Spacemacs like runner, where you hit one key and get send down a tree
of successive keys to execute a command / shortcut you have previously
configured.

# Setup

* Clone it
* `cargo build --release` inside the cloned directory
* Copy `config.json` to your users config directory (for example to `~/.config/spacerun/config.json` for linux)
* Create a keyboard shortcut in your system to start `<spacerun-dir>/target/release/spacerun`
* Edit the JSON file with your shortcuts
* Have fun :rocket:

# ToDos

* [ ] Generate default config file if it is missing
* [ ] Replace `unwrap()`s with real error handling & good error messages
  * [ ] Errors
      * [ ] On config missing / unreadable
  * [ ] Show errors in window, it should not fail to show at least something when run
* [ ] Add styling config options
  * [ ] Font config
      * [/] Size
  * [ ] colors
* [ ] Global command layer for configurable hotkeys
  * [ ] Backspace to go back up one layer
  * [ ] Escape exits

## Cool & shiny things to implement

* [ ] Auto-generate a form by placeholders in the command (e.g. `${'My Field Name': default val}`)
* [ ] Replace placeholders by data / vars (e.g. `${clipboard}` / `${clipboard_selection}`)
  (Although this is a bit redundant with scripts being able to fetch the same information)
* [ ] Better / cooler / easier to edit format instead of a json file?
* [ ] Form to add / edit commands in-program
* [ ] Repeating- / multi-mode, where multiple commands can be executed once
* [ ] CLI interface to start from specific subtree / specify different config path
* [ ] Async mode, where the window pops up again after a command has finished to give more options
      (Also solvable by using a CLI interface (not yet implemented), but not as cool?)
* [ ] Server mode, instance is constantly running in background so the JSON won't need to be parsed
      each time the window should be displayed.
* [ ] Breadcrumbs, showing the path you went down.
* [ ] Show name and description of the current tree as a title / subtitle of the window
* [ ] Radial menu instead of list as option (Because radial menus are awesome!)
* [ ] More key-value pairs for command leafs!
    * [ ] "description" to find / understand your nodes & commands, even after a long night.
    * [ ] "clip" copying a string to clipboard.
