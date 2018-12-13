# Idea

Emacs / Spacemacs like runner, where you hit one key and get send down a tree
of successive keys to execute a command / shortcut you have previously
specified.

# ToDos

* [ ] Implement generic package installment
  * [ ] Configure font & load generically (not by hardcoded path)
  * [ ] Generate default config file on installation
* [ ] Replace `unwrap()`s with real error handling & good error messages
  * [ ] On config missing / unreadable
  * [ ] On font missing (add fallback?)
* [ ] Add styling config options

## Cool & shiny things to implement

* [ ] Auto-generate a form by placeholders in the command (e.g. `${'My Field Name': default val}`)
* [ ] Replace placeholders by data / vars (e.g. `${clipboard}` / `${clipboard_selection}`)
  (Although this is a bit redundant with scripts being able to fetch the same information)
* [ ] Better / cooler / easier to edit format instead of a json file?
