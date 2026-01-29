// Script to generate the SVG icon for Thai ID Middleware
// The SVG should be 512x512 for electron-builder compatibility
// Run with: node generate-icon.js
//
// Note: The PNG icon is generated during the CI/CD build process using:
// - macOS: rsvg-convert (librsvg)
// - Windows: Inkscape
//
// To generate PNG locally, install one of:
// - macOS: brew install librsvg && rsvg-convert -w 512 -h 512 icon.svg -o icon.png
// - Linux: apt install librsvg2-bin && rsvg-convert -w 512 -h 512 icon.svg -o icon.png
// - Windows: choco install inkscape && inkscape --export-type=png --export-filename=icon.png --export-width=512 --export-height=512 icon.svg

const fs = require('fs')

// 512x512 SVG icon with purple/blue gradient and card symbol
const svgIcon = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 512 512" width="512" height="512">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#667eea"/>
      <stop offset="100%" style="stop-color:#764ba2"/>
    </linearGradient>
  </defs>
  <rect width="512" height="512" rx="96" fill="url(#bg)"/>
  <rect x="96" y="128" width="320" height="224" rx="32" fill="white" opacity="0.9"/>
  <rect x="128" y="160" width="128" height="96" rx="16" fill="#667eea"/>
  <rect x="128" y="288" width="256" height="24" rx="8" fill="#ccc"/>
  <circle cx="352" cy="416" r="48" fill="white" opacity="0.9"/>
  <circle cx="352" cy="416" r="24" fill="#667eea"/>
</svg>`

fs.writeFileSync('icon.svg', svgIcon)
console.log('SVG icon created: icon.svg (512x512)')
console.log('')
console.log('To generate PNG, run one of:')
console.log('  macOS/Linux: rsvg-convert -w 512 -h 512 icon.svg -o icon.png')
console.log('  Windows:     inkscape --export-type=png --export-filename=icon.png --export-width=512 --export-height=512 icon.svg')
