# Themedex

[Project in early development]

A desktop theme cataloguer and selector, written in Rust.

## Development

Build core functionality first, add UI later

### To be determined

+ Colour scheme storage format? How many colours? Optional highlight colours?

+ How is a colour scheme applied?
    + This should be configurable for a user's system

### Functionality to implement

#### Adding to database

+ Bulk add colour schemes
    * Perhaps just handle in CLI with shell path expansion

#### Applying themes

+ Take a colour scheme and generate the necessary files for configs
    * Generate a range of files in .cache/themedex
    * A css file with 16 colours, bg, fg, hl, etc.
    * Files for each tool, e.g. neovim, ghostty
    * These can be symlinked by the user into the necessary locations
+ Apply wallpaper via user-specified command (swww default)

#### Browsing themes

+ List themes via command line with various filters

#### Further functionality

+ Colour scheme likeness engine
    * Add wallpaper or variant in interactive mode, it is compared against each
      colour scheme to give a percentage match in order to pick the most
      appropriate theme
+ Wallpaper variant generator
    * Create additional variants for existing wallpapers, converting colours to
      match a colour scheme
    * Likely use an existing tool for this (dipc might be ideal as a rust
      package, alternately gowall might work)

## Colour scheme storage

Core JSON for each scheme will consist of:
+ name (not strictly necessary?)
+ variant (light/dark)
+ system (either base16 or base24)
+ palette
    * a dict of 16 or 24 colours
+ tools
    * optional, a dict with names of colour schemes to be provided to individual tool
      configs
+ extra
    * optional, a dict with additional colours. Probably limited functionality
      for this but could perhaps be included in dipc colours

This should provide enough information 
For Dark Themes:
    color00: Default Background.
    color05: Default Foreground (Main Text).
    color06: Cursor / Large highlighting.
    color01: Lighter Background (Status bars, line highlights).
    color07: "Brightest" white (extra pop).

For Light Themes:
    color00: Default Background (usually white/off-white).
    color05: Default Foreground (usually dark grey/black).
    color06: Cursor / Selection.

Approximate mappings to primary colours (not reliable at all!)
Black: base00
Red: base08
Green: base0B
Yellow: base0A
Blue: base0D
Magenta: base0E
Cyan: base0C
White: base05

Want to scrape themes from
https://github.com/tinted-theming/schemes
Include a separate tool for this?
