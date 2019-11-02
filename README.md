**NOTE** Due to either the state of current Rust GUI libs being a little incomplete I need to test out some libs to see which support the feature-set I need. 
*It can't be that hard to automatically focus an input field*. Or can it? :suspect:

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

## Configuration

Edit `config.json` in your configuration path to change spaceruns behaviour.

* `commands` contains your shortcuts and the action it should execute.
* `font_size` sets the displayed fonts size (default is 14).
* `position` specifies the position of spaceruns window.
  Its value can be one of:
  * `centered` for a centered window with automatic width  (default).
  * `top` for a full-width window positioned at the top of your screen.
  * `bottom` for a full-width window positioned at the bottom of your screen.

# Principles

* Spacerun should be fast to use.
* Spacerun is not only a task runner, but a productivity tool.
* Automation is awesome; Automation is both documenting and time saving.
  Therefore, spacerun broadens the possible applications of automation by enabling rapid changes to it.

# ToDos

* [ ] **Blocking** Switch to a framework that supports input focusing, as I am unable to do so with the current one. (Maybe try Azul, as its the cool new kid on the block?)

* [ ] Generate default config file if it is missing
* [ ] Replace `unwrap()`s with real error handling & good error messages
  * [ ] Errors
      * [ ] On config missing / unreadable
  * [ ] Show errors in window, it should not fail to show at least *something* when run
* [ ] Add styling config options
  * [ ] Font config
      * [x] Size
  * [ ] colors
* [ ] Global command layer for configurable hotkeys
  * [ ] Backspace to go back up one layer
  * [ ] Escape exits
* Refactor code & add some tests

## Cool & shiny things to implement

* [x] Auto-generate a form by placeholders in the command (e.g. `${'My Field Name': default val}`)
* [ ] Context sensitive commands
    * [ ] Prev focused window. I could have added a `"class": "^chromium"` to the commands config.
          This option will only show if chromium was focused previous to spacerun being opened.
          For more usability we could add placeholders in the command for window_id & more.
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
* [x] Breadcrumbs, showing the path you went down.
* [ ] Show name and description of the current tree as a title / subtitle of the window
* [ ] Radial menu instead of list as option (Because radial menus are awesome!)
* [ ] Unicode / emoticons / ligatures / FontAwesome support
* [ ] dmenu parameter mode (similar to rofi)
* [ ] More key-value pairs for command leafs!
    * [ ] "description" to find / understand your nodes & commands, even after a long night.
    * [ ] "clip" copying a string to clipboard.
    * [ ] "repeat" Repeating last command. Values include:
        * "global" - Repeat last command, regardless which command it was
        * "subtree" - Repeat last command executed in the subtree of the command node
        * "siblings" - Repeat last command executed only in the direct children of the command node

