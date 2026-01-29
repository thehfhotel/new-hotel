# Icon Generation

The `icon.svg` file contains the source icon for the Thai ID Middleware app.

## Required Icon Files

Tauri requires the following icon files for bundling:

- `icon.png` - 512x512 PNG for tray icon
- `32x32.png` - 32x32 PNG
- `128x128.png` - 128x128 PNG
- `128x128@2x.png` - 256x256 PNG (for retina displays)
- `icon.icns` - macOS app icon
- `icon.ico` - Windows app icon

## Generating Icons

### Option 1: Using librsvg (recommended)

Install librsvg:
```bash
# macOS
brew install librsvg

# Linux
apt install librsvg2-bin
```

Generate PNG icons:
```bash
cd src-tauri/icons
rsvg-convert -w 512 -h 512 icon.svg -o icon.png
rsvg-convert -w 32 -h 32 icon.svg -o 32x32.png
rsvg-convert -w 128 -h 128 icon.svg -o 128x128.png
rsvg-convert -w 256 -h 256 icon.svg -o 128x128@2x.png
```

### Option 2: Using Tauri CLI

The Tauri CLI can generate all required icons from a source PNG:
```bash
# First create a 512x512 PNG, then:
npm run tauri icon icon.png
```

### Option 3: Using Inkscape

```bash
# Windows
inkscape --export-type=png --export-filename=icon.png --export-width=512 --export-height=512 icon.svg
```

## Creating .icns and .ico Files

After generating PNG files, use the Tauri CLI:
```bash
npm run tauri icon icon.png
```

Or use online converters / image editing software to create the platform-specific icons.
