# Logitech G600 Mouse Configuration Tool For Linux

Logitech G600 is a configurable mouse with 20 buttons and built-in memory for 3 profiles of button, resolution and color assignments.  This tool supports dumping/restoring those profiles, as well as converting them to YAML text for editing.

![G600](https://s21.q4cdn.com/947125427/files/images/news/2012/Logitech_G600_MMO_Gaming_Mouse_webready.jpg)

## Configuration format

See [config/default.yaml](config/default.yaml) for example.

The configuration looks like this:

```yaml
profiles:
- {{profile 1}}
- {{profile 2}}
- {{profile 3}}
```

Each profile defines LED color setup, USB report rate, DPI configuration, and button assigments:

```yaml
profiles:
- led_...: ...
  report_rate: ...
  dpi_...: ...
  buttons: ...
  g_led_color: ...
  g_buttons: ...
- ...
- ...
```

### USB report rate

```yaml
  report_rate: Hz1000
```

`report_rate` defines how frequently the mouse reports its position offset when it moves. You can leave it at `Hz1000` (1000 times per second) or change it to `Hz500`, `Hz333`, `Hz250`, `Hz200`, `Hz166`, `Hz142`, or `Hz125`.

### LED color setup

LED color setup looks like this:

```yaml
- led_color: '000000'
  led_effect: Cycle
  led_duration: 4
  g_led_color: '000000'
```

`led_effect` is `Solid`, `Breath` or `Cycle`.

When it is `Solid`, `led_duration` has no effect and the LED color is defined by an RGB hex string in `led_color` (between `'000000'`, which needs to be quoted in YAML, and `FFFFFF`). When a button with `GShift` action is pressed, the color changes to `g_led_color`.

When it is `Breath`, LED color slowly changes between `led_color` (or `g_led_color` with `GShift`) and total black and quickly goes back during `led_duration + 1` seconds, where `led_duration` goes from 0 upto 255.

When it is `Cycle`, `led_color` and `g_led_color` have no effect and the LED color passes through the color cycle (starting from red) during `(led_duration + 1) * 5 / 3` seconds.

## DPI configuration

DPI configuration looks like this:

```yaml
  dpi_shift: 400
  dpi_default: 4
  dpis:
  - 400
  - 1200
  - 2000
  - 3200
```

`dpi_shift` defines the DPI when a button with `DPIShift` action is pressed. It ranges from 200 upto 8200 with a step of 50.

`dpis` is a list of 4 entries. 0 entries are ignored, others (200, 250, ..., 8200) define DPIs for this profile.

`dpi_default` ranges from 1 upto 4 and chooses which DPI from `dpis` list is used when you switch to this profile.

## Button assigments

`buttons` and `g_buttons` are lists of 20 entries each, one for each of the mouse buttons. The first 6 actions typically are:

```yaml
  buttons:
  - action: LeftClick
  - action: RightClick
  - action: WheelClick
  - action: WheelLeft
  - action: WheelRight
  - action: GShift # or DPIShift
```

The rest of the buttons are labelled on the mouse from G7 upto G20.

While a button with `GShift` action is pressed, other button presses are interpreted according to `g_buttons` configuration.

Each button entry is either an `action` or a `key` with `modifiers`. Possible actions are those listed above and also:

- `M10`, `M11`, ..., `M20`: mouse button 10, 11, ..., 20
- `ProfileCycle`: switch to the next of 3 mouse profiles (and cycle back to the first)
- `DPICycle`: switch to the next DPI in the `dpis` list (and cycle back to the first DPI)
- `DPIUp`: switch to the next DPI in the `dpis` list (and stay at the last defined DPI)
- `DPIDown`: switch to the previous DPI in the `dpis` list (and stay at the first defined DPI)
- `DPIDefault`: switch to the `dpi_default` DPI in the `dpis` list
- `DPIShift`: work at the `dpi_shift` DPI while this button is pressed

`modifiers` is a list that may include `Ctrl`, `Shift`, `Alt`, `Meta`, `RCtrl`, `RShift`, `RAlt`, `RMeta`.

`key` is one of the HID key names (with underscore `_` before digit key names): `ErrorRollOver` `POSTFail` `ErrorUndefine` `A` `B` `C` `D` `E` `F` `G` `H` `I` `J` `K` `L` `M` `N` `O` `P` `Q` `R` `S` `T` `U` `V` `W` `X` `Y` `Z` `_1` `_2` `_3` `_4` `_5` `_6` `_7` `_8` `_9` `_0` `Enter` `Esc` `Backspace` `Tab` `Spacebar` `Minus` `Equal` `LeftBrace` `RightBrace` `Backslash` `Backslash1` `Semicolon` `Apostrophe` `Grave` `Comma` `Period` `Slash` `CapsLock` `F1` `F2` `F3` `F4` `F5` `F6` `F7` `F8` `F9` `F10` `F11` `F12` `PrintScreen` `ScrollLock` `Pause` `Insert` `Home` `PageUp` `Delete` `End` `PageDown` `Right` `Left` `Down` `Up` `NumLock` `KPSlash` `KPAsterisk` `KPMinus` `KPPlus` `KPEnter` `KP1` `KP2` `KP3` `KP4` `KP5` `KP6` `KP7` `KP8` `KP9` `KP0` `KPDot` `Backslash2` `Compose` `Power` `KPEqual` `F13` `F14` `F15` `F16` `F17` `F18` `F19` `F20` `F21` `F22` `F23` `F24` `Open` `Help` `Menu` `Select` `Stop` `Again` `Undo` `Cut` `Copy` `Paste` `Find` `Mute` `VolumeUp` `VolumeDown` `LockingCapsLock` `LockingNumLock` `LockingScrollLock` `KPComma` `KPEqual2` `Kanji1` `Kanji2` `Kanji3` `Kanji4` `Kanji5` `Kanji6` `Kanji7` `Kanji8` `Kanji9` `Lang1` `Lang2` `Lang3` `Lang4` `Lang5` `Lang6` `Lang7` `Lang8` `Lang9` `AlternateErase` `SysReq` `Cancel` `Clear` `Prior` `Return` `Separator` `Out` `Oper` `ClearAgain` `CursorSelect` `ExtendSelection` `A5` `A6` `A7` `A8` `A9` `AA` `AB` `AC` `AD` `AE` `AF` `KP00` `KP000` `ThousandsSeparator` `DecimalSeparator` `CurrencyUnit` `CurrencySubunit` `KPLeftParen` `KPRightParen` `KPLeftCurly` `KPRightCurly` `KPTab` `KPBackspace` `KPa` `KPb` `KPc` `KPd` `KPe` `KPf` `KPXor` `KPCircumflex` `KPPercent` `KPLessThan` `KPGreaterThan` `KPAmpersand` `KPDoubleAmpersand` `KPBar` `KPDoubleBar` `KPColor` `KPHash` `KPSpace` `KPAt` `KPExclamation` `KPMemoryStore` `KPMemoryRecall` `KPMemoryClear` `KPMemoryAdd` `KPMemorySubtract` `KPMemoryMultiply` `KPMemoryDivide` `KPPlusMinus` `KPClear` `KPClearEntry` `KPBinary` `KPOctal` `KPDecimal` `KPHexadecimal` `HidDE` `HidDF` `LeftCtrl` `LeftShift` `LeftAlt` `LeftMeta` `RightCtrl` `RightShift` `RightAlt` `RightMeta` `MediaPlaypause` `MediaStopCD` `MediaPreviousSong` `MediaNextSong` `MediaEjectCD` `MediaVolumeUp` `MediaVolumeDown` `MediaMute` `MediaWWW` `MediaBack` `MediaForward` `MediaStop` `MediaFind` `MediaScrollUp` `MediaScrollDown` `MediaEdit` `MediaSleep` `MediaCoffee` `MediaRefresh` `MediaCalc` `HidFC` `HidFD` `HidFE` `HidFF`.

Both `modifiers` and `keys` are akin to physical keys on a QWERTY keybooard, so when they are remappend or the current keyboard layout is not QWERTY, the logical key press is a translation of the physical key press. For example, pressing mouse keys assigned to `A` `S` `D` `F` produces `asdf` when the current layout is QWERTY, but `aoeu` when it is Dvorak.
