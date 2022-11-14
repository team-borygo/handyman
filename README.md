# Handyman - store bookmarks to anything

Handyman
Acronym's
Noticeably
Dumb,
Yet
Makes
A
Name

## The motivation

Internet browsers have bookmarks. File managers have bookmars.
Sometimes terminal emulator have bookmarks.

Besides that we also want to store notes or various stuff for quick access (like blockchain address).

What if we could have one tool to unite all bookmarks, that could be easily ported to other computers, backed up, and wired into Unix pipeline philosophy so it's easy extendable?

**Handyman** is created to do just that.

It connects to operating system clipboard, and (on request) saves clipboard content (or just selection!) as bookmark, that later on can be brought to life through menu application (like *dmenu* or *rofi*). It can be used in graphical or terminal-only environments.

As browser bookmarks replacement Handyman automatically downloads page title for URL bookmarks, and automatically uses internet browsers to open them.

As file manager bookmarks replacement Handyman automatically uses file manager/terminal emulator/dedicated application to open locations on file system.

Traditional "copy to clipboard" (or automated pasting if physically possible) can be used as well.

Everything is wired up using shortcuts that you can assign as you like in your shortcut manager. 

## Roadmap

1. [ ] Basic usage through CLI-only usage
2. [ ] dmenu/rofi integration
3. [ ] fuzzy search, text proximity scoring (to allow typos) - may not be available if used with dmenu/rofi
4. [ ] Automated actions on selected item (open in browser/terminal emulator, move in current terminal session etc.)

