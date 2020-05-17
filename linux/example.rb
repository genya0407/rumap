# right alt -> alt
# capslock -> ctrl

remap 'C-h', to: 'Left', with_modifier: 'Shift'
remap 'C-j', to: 'Down', with_modifier: 'Shift'
remap 'C-k', to: 'Up', with_modifier: 'Shift'
remap 'C-l', to: 'Right', with_modifier: 'Shift'
remap 'C-BackSpace', to: 'Delete'

# mac
remap 'Alt-a', to: 'C-a'

line = 'crx_ophjlpahpchlmihnnnihgmmeilfjmjjc'
window class_only: %w[chromium] + [line] do
  %w[z x c v w t f Return].each do |key|
    remap "Alt-#{key}", to: "C-#{key}"
  end
end

window class_only: 'code-oss' do
  %w[s p z x c v w t f d slash Return comma].each do |key|
    remap "Alt-#{key}", to: "C-#{key}"
  end
end

window class_only: 'konsole' do
  remap 'Alt-c', to: 'C-Shift-c'
  remap 'Alt-v', to: 'C-Shift-v'
  remap 'Alt-w', to: 'C-Shift-w'
  remap 'Alt-t', to: 'C-Shift-t'
  remap 'Alt-d', to: 'C-parenleft'
end

# utilities

remap 'XF86XK_AudioLowerVolume', to: execute('pamixer --decrease 5')
remap 'XF86XK_AudioRaiseVolume', to: execute('pamixer --increase 5')
remap 'XF86XK_MonBrightnessDown', to: execute('sudo light -U 5')
remap 'XF86XK_MonBrightnessUp', to: execute('sudo light -A 5')
remap 'XF86XK_Tools', to: execute('deepin-screenshot')
