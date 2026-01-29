# Thai ID Middleware

Cross-platform desktop app with GUI that runs an HTTP middleware for Thai ID card reading.

## Features

- **GUI Status Display**: Reader connection, card insertion, read progress
- **HTTP Server**: Exposes `/health` and `/read` on localhost:9898
- **System Tray**: Minimize to tray, background operation
- **Cross-Platform**: Windows, macOS, Linux
- **No Installation Required**: Portable executable

## Requirements

- Smart card reader (PC/SC compatible)
- Thai National ID card
- pcscd service running (Linux)
- Node.js 18+ and npm

### Build Requirements

The `pcsclite` package requires native compilation. Install these dependencies before running `npm install`:

**Ubuntu/Debian:**
```bash
sudo apt-get install build-essential libpcsclite-dev pcscd pcsc-tools
```

**macOS:**
```bash
# PC/SC is built into macOS, no additional install needed
xcode-select --install  # For build tools
```

**Windows:**
```bash
# Install Windows Build Tools
npm install --global windows-build-tools
# PC/SC is built into Windows
```

### Linux Setup

```bash
# Install PC/SC daemon and development libraries
sudo apt-get install pcscd pcsc-tools libpcsclite-dev

# Start the service
sudo systemctl start pcscd
sudo systemctl enable pcscd

# Test card reader
pcsc_scan
```

## Development

```bash
# Install dependencies
npm install

# Run in development mode
npm start
```

## Building

```bash
# Build for all platforms
npm run build

# Platform-specific builds
npm run build:win    # Windows portable .exe
npm run build:mac    # macOS .dmg
npm run build:linux  # Linux .AppImage
```

## API Endpoints

### GET /health

Check server and reader status.

```bash
curl http://localhost:9898/health
```

Response:
```json
{
  "status": "ok",
  "timestamp": "2026-01-29T10:00:00.000Z",
  "readerConnected": true,
  "cardInserted": true,
  "serverRunning": true,
  "port": 9898,
  "readerName": "ACS ACR122U"
}
```

### GET /read

Read Thai ID card data.

```bash
curl http://localhost:9898/read
```

Response:
```json
{
  "success": true,
  "data": {
    "cid": "1234567890123",
    "thaiTitle": "นาย",
    "thaiFirstName": "ชื่อ",
    "thaiLastName": "นามสกุล",
    "englishTitle": "Mr.",
    "englishFirstName": "FirstName",
    "englishLastName": "LastName",
    "dateOfBirth": "1990-01-15",
    "gender": "Male",
    "address": "123 ถนน ตำบล อำเภอ จังหวัด 10000",
    "issueDate": "2020-01-01",
    "expireDate": "2028-01-01"
  }
}
```

## Usage

1. Download the executable for your platform
2. Run the application
3. Connect your smart card reader
4. Insert a Thai ID card
5. Web applications can now read card data from `http://localhost:9898`

## Troubleshooting

### Reader not detected

- Ensure pcscd service is running (Linux)
- Try unplugging and replugging the reader
- Check if reader is recognized: `lsusb` or `pcsc_scan`

### Card read errors

- Ensure the card is fully inserted
- Clean the card contacts
- Try a different card to rule out card damage

## License

MIT
