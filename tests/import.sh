exe=$1

$exe add-color-scheme tests/schemes/catppuccin-mocha.json
$exe add-color-scheme tests/schemes/tokyo-night-storm.json

$exe add-wallpaper --name purple_pyramid tests/tempwalls/purple_pyramid/catppuccin-mocha.jpg catppuccin-mocha --update
$exe add-wallpaper --name sunset_lines tests/tempwalls/sunset_lines/catppuccin-mocha.png catppuccin-mocha --update

$exe add-wallpaper-version tests/tempwalls/purple_pyramid/tokyo-night-storm.png purple_pyramid tokyo-night-storm --update
$exe add-wallpaper-version tests/tempwalls/sunset_lines/tokyo-night-storm.png sunset_lines tokyo-night-storm --update
