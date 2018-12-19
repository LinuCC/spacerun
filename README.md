# Idea

Emacs / Spacemacs like runner, where you hit one key and get send down a tree
of successive keys to execute a command / shortcut you have previously
specified.

# ToDos

* [ ] Generate default config file if it is missing
* [ ] Replace `unwrap()`s with real error handling & good error messages
  * [ ] On config missing / unreadable
  * [ ] On font missing (add fallback?)
* [ ] Add styling config options
  * [ ] Font config
* [ ] Backspace to go back up one layer

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
* [ ] More key-value pairs for command leafs!
    * [ ] "description" to find / understand your nodes & commands, even after a long night.
    * [ ] "clip" copying a string to clipboard.
