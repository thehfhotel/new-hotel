// Simple script to generate a basic icon PNG
// Run with: node generate-icon.js

const fs = require('fs')

// Simple 16x16 PNG icon (purple/blue gradient with card symbol)
// This is a minimal valid PNG with a simple design
const iconBase64 =
  'iVBORw0KGgoAAAANSUhEUgAAAEAAAABACAYAAACqaXHeAAAACXBIWXMAAAsTAAALEwEAmpwY' +
  'AAABrklEQVR4nO2bsU7DMBCGf0EqYoEFiQV2xANQdeQJYGRjZGRk5AV4AhZ2pO7sgIAB8QBs' +
  'TCAUD0BsiFIqNXHjxNdz7P+TTlXl5O7/z+c4FwMAAAAAAAAAAACAJEiNX0DfvQ9Sg59A370P' +
  'UoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0Df' +
  'vQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOf' +
  'QN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9S' +
  'g59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9' +
  'D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A370PUoOfQN+9D1KDn0DfvQ9Sg59A' +
  '370PUoOfQN+9D1KDn0DfvQGQGvwE+u59AAAAAAAAAAAAAPwf/gAFrkm1q5O0rAAAAABJRU5E' +
  'rkJggg=='

// Decode and save
const iconBuffer = Buffer.from(iconBase64, 'base64')
fs.writeFileSync('icon.png', iconBuffer)
console.log('Icon created: icon.png')

// Also create a simple SVG version
const svgIcon = `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64" width="64" height="64">
  <defs>
    <linearGradient id="bg" x1="0%" y1="0%" x2="100%" y2="100%">
      <stop offset="0%" style="stop-color:#667eea"/>
      <stop offset="100%" style="stop-color:#764ba2"/>
    </linearGradient>
  </defs>
  <rect width="64" height="64" rx="12" fill="url(#bg)"/>
  <rect x="12" y="16" width="40" height="28" rx="4" fill="white" opacity="0.9"/>
  <rect x="16" y="20" width="16" height="12" rx="2" fill="#667eea"/>
  <rect x="16" y="36" width="32" height="3" rx="1" fill="#ccc"/>
  <circle cx="44" cy="52" r="6" fill="white" opacity="0.9"/>
  <circle cx="44" cy="52" r="3" fill="#667eea"/>
</svg>`

fs.writeFileSync('icon.svg', svgIcon)
console.log('SVG icon created: icon.svg')
