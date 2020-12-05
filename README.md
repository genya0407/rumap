# rumap

Keymapper for X Window System. Inspired by [xremap](https://github.com/k0kubun/xremap).

## Keymap configuration

You can write keymap configurations with Ruby DSL:

```ruby
# "global" keymaps
remap 'Control-BackSpace', to: 'Delete'

# execute command
remap 'Alt-Shift-4', to: execute('gnome-screenshot -a -d 0')

# application specific keymaps
window class_only: %w[chromium discord] do
  %w[r z x c v w t f Return].each do |key|
    remap 'Alt_L', to: 'Control_L'
    remap "Alt-#{key}", to: "C-#{key}"
  end
end

# vim-like arrow bindings
# map `Control-h` to `Left`, and `Control-Shift-h` to `Shift-Left`, and so on.
remap 'Control-h', to: 'Left', with_modifier: 'Shift'
remap 'Control-j', to: 'Down', with_modifier: 'Shift'
remap 'Control-k', to: 'Up', with_modifier: 'Shift'
remap 'Control-l', to: 'Right', with_modifier: 'Shift'
```

## Installation

Download binary from [Release](https://github.com/genya0407/rumap/releases) page, and locate `rumap` under `$PATH`.

## Start keymapping

```shell
# Write configuration
$ vim ~/.rumap
# launch rumap
$ rumap --xremap-config ~/.rumap
```
