# Experimental Wayland window dragging

This fork adds an opt-in `:window-drag` boolean property to Eww's `eventbox`.
It is designed for **one Wayland monitor** and moves the **whole Eww window**,
not individual child widgets. It has no external helper process.

## Yuck example

```yuck
(defvar edit-mode false)

(defwindow music
  :monitor 0
  :stacking "fg"
  :focusable false
  :geometry (geometry
    :x "40px"
    :y "40px"
    :width "300px"
    :height "180px"
    :anchor "top left")
  (eventbox
    :window-drag edit-mode
    (music-player)))
```

You can also wrap an existing widget with a dedicated outer `eventbox` in an
existing `defwindow`. The drag handle moves that entire window, including
anything else in it.

## Toggle from a Hyprland keybinding

Eww's `eww update` can set the variable directly:

```sh
eww update edit-mode=true
eww update edit-mode=false
```

To toggle using a single keybinding, create `~/.config/eww/scripts/drag-toggle.sh`:

```sh
#!/bin/sh
current=$(eww get edit-mode) || exit 1
case "$current" in
  true) eww update edit-mode=false ;;
  *)    eww update edit-mode=true ;;
esac
```

Then bind your preferred key to that script in your compositor configuration.
For Hyprland's Lua configuration (0.56), for example:

```lua
hl.bind("SUPER + ALT + D", hl.dsp.exec_cmd("sh ~/.config/eww/scripts/drag-toggle.sh"))
```

The variable is daemon-global, so keep the window-specific state in the
window's outer drag handle if more windows are added later.

## Expected behavior

- `edit-mode=false`: child buttons and sliders receive normal input.
- `edit-mode=true`: hold left mouse button over the drag eventbox and move.
  The transparent eventbox takes mouse input ahead of its child widgets.
- Toggle off: the window keeps its in-memory position and controls work again.
- Close and reopen the window: the original Yuck geometry is used again.
- Drag changes the current window's Wayland layer-shell margins; it does not
  write to the config or spawn shell commands on mouse motion.

## Limitations (initial version)

- Wayland / `gtk-layer-shell` only; the property has no effect on X11.
- Single monitor; no crossing outputs, snapping, resizing or persistence.
- Use a **dedicated outer eventbox** as the drag handle, not an eventbox
  containing separate interaction handlers at the same level.
- A window's positioning is switched to top-left anchoring at the first drag.
- This is an experimental implementation and needs a GTK/Wayland runtime test.
