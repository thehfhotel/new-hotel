let pcsc = null
let reader = null
let cardInserted = false
let readerConnected = false
let statusCallback = () => {}
let currentProtocol = null
let pcscliteAvailable = false

// Try to load pcsclite - it may not be available on all systems
let pcsclite = null
try {
  pcsclite = require('pcsclite')
  pcscliteAvailable = true
} catch (err) {
  console.warn('PC/SC library not available:', err.message)
  console.warn('Card reading will be disabled. Install libpcsclite-dev and rebuild.')
}

// Thai National ID Card APDU Commands
const APDU = {
  // Select Thai ID Card application
  SELECT_APPLET: Buffer.from([0x00, 0xA4, 0x04, 0x00, 0x08, 0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x01]),

  // Read CID (Citizen ID) - 13 digits
  READ_CID: Buffer.from([0x80, 0xB0, 0x00, 0x04, 0x02, 0x00, 0x0D]),

  // Read Thai name
  READ_TH_FULLNAME: Buffer.from([0x80, 0xB0, 0x00, 0x11, 0x02, 0x00, 0x64]),

  // Read English name
  READ_EN_FULLNAME: Buffer.from([0x80, 0xB0, 0x00, 0x75, 0x02, 0x00, 0x64]),

  // Read Date of Birth
  READ_DOB: Buffer.from([0x80, 0xB0, 0x00, 0xD9, 0x02, 0x00, 0x08]),

  // Read Gender
  READ_GENDER: Buffer.from([0x80, 0xB0, 0x00, 0xE1, 0x02, 0x00, 0x01]),

  // Read Address
  READ_ADDRESS: Buffer.from([0x80, 0xB0, 0x15, 0x79, 0x02, 0x00, 0x64]),

  // Read Issue Date
  READ_ISSUE_DATE: Buffer.from([0x80, 0xB0, 0x01, 0x67, 0x02, 0x00, 0x08]),

  // Read Expire Date
  READ_EXPIRE_DATE: Buffer.from([0x80, 0xB0, 0x01, 0x6F, 0x02, 0x00, 0x08])
}

function initCardReader(callback) {
  statusCallback = callback

  if (!pcscliteAvailable) {
    console.warn('Card reader initialization skipped - pcsclite not available')
    return
  }

  try {
    pcsc = pcsclite()

    pcsc.on('reader', (r) => {
      reader = r
      readerConnected = true
      console.log(`Reader detected: ${r.name}`)
      statusCallback(getStatus())

      r.on('status', (status) => {
        const changes = r.state ^ status.state

        if (changes) {
          if ((changes & r.SCARD_STATE_EMPTY) && (status.state & r.SCARD_STATE_EMPTY)) {
            // Card removed
            cardInserted = false
            console.log('Card removed')
            statusCallback(getStatus())
          } else if ((changes & r.SCARD_STATE_PRESENT) && (status.state & r.SCARD_STATE_PRESENT)) {
            // Card inserted
            cardInserted = true
            console.log('Card inserted')
            statusCallback(getStatus())
          }
        }
      })

      r.on('end', () => {
        console.log('Reader disconnected')
        reader = null
        readerConnected = false
        cardInserted = false
        statusCallback(getStatus())
      })

      r.on('error', (err) => {
        console.error('Reader error:', err.message)
      })
    })

    pcsc.on('error', (err) => {
      console.error('PCSC error:', err.message)
      readerConnected = false
      statusCallback(getStatus())
    })

  } catch (err) {
    console.error('Failed to initialize card reader:', err.message)
    readerConnected = false
  }
}

function getStatus() {
  return {
    readerConnected,
    cardInserted,
    serverRunning: true,
    port: 9898,
    readerName: reader ? reader.name : null
  }
}

function transmit(protocol, command) {
  return new Promise((resolve, reject) => {
    reader.transmit(command, 256, protocol, (err, data) => {
      if (err) {
        reject(err)
      } else {
        resolve(data)
      }
    })
  })
}

function parseThaiString(buffer) {
  // Remove status bytes (last 2 bytes) and trailing spaces/nulls
  const data = buffer.slice(0, -2)
  // Thai text is TIS-620 encoded, but we'll try UTF-8 first
  let str = data.toString('utf8').replace(/\x00/g, '').trim()
  // Replace # with space (common separator in Thai ID cards)
  str = str.replace(/#/g, ' ').replace(/\s+/g, ' ').trim()
  return str
}

function parseAsciiString(buffer) {
  const data = buffer.slice(0, -2)
  return data.toString('ascii').replace(/\x00/g, '').replace(/#/g, ' ').replace(/\s+/g, ' ').trim()
}

function parseDate(buffer) {
  // Date format: YYYYMMDD
  const dateStr = buffer.slice(0, -2).toString('ascii')
  if (dateStr.length >= 8) {
    const year = dateStr.substring(0, 4)
    const month = dateStr.substring(4, 6)
    const day = dateStr.substring(6, 8)
    return `${year}-${month}-${day}`
  }
  return dateStr
}

async function readCard() {
  if (!reader) {
    throw new Error('No card reader connected')
  }

  if (!cardInserted) {
    throw new Error('No card inserted')
  }

  return new Promise((resolve, reject) => {
    reader.connect({ share_mode: reader.SCARD_SHARE_SHARED }, async (err, protocol) => {
      if (err) {
        return reject(new Error(`Connection failed: ${err.message}`))
      }

      currentProtocol = protocol

      try {
        // Select Thai ID applet
        const selectResponse = await transmit(protocol, APDU.SELECT_APPLET)
        const sw = selectResponse.slice(-2)
        if (sw[0] !== 0x90 || sw[1] !== 0x00) {
          throw new Error('Failed to select Thai ID applet. Is this a Thai National ID card?')
        }

        // Read all data
        const [cidResp, thNameResp, enNameResp, dobResp, genderResp, addrResp, issueDateResp, expireDateResp] =
          await Promise.all([
            transmit(protocol, APDU.READ_CID),
            transmit(protocol, APDU.READ_TH_FULLNAME),
            transmit(protocol, APDU.READ_EN_FULLNAME),
            transmit(protocol, APDU.READ_DOB),
            transmit(protocol, APDU.READ_GENDER),
            transmit(protocol, APDU.READ_ADDRESS),
            transmit(protocol, APDU.READ_ISSUE_DATE),
            transmit(protocol, APDU.READ_EXPIRE_DATE)
          ])

        // Parse responses
        const cid = parseAsciiString(cidResp)
        const thaiName = parseThaiString(thNameResp)
        const englishName = parseAsciiString(enNameResp)
        const dateOfBirth = parseDate(dobResp)
        const gender = parseAsciiString(genderResp)
        const address = parseThaiString(addrResp)
        const issueDate = parseDate(issueDateResp)
        const expireDate = parseDate(expireDateResp)

        // Split names
        const thaiNameParts = thaiName.split(' ').filter(Boolean)
        const englishNameParts = englishName.split(' ').filter(Boolean)

        const result = {
          cid,
          thaiTitle: thaiNameParts[0] || '',
          thaiFirstName: thaiNameParts[1] || '',
          thaiLastName: thaiNameParts[2] || '',
          englishTitle: englishNameParts[0] || '',
          englishFirstName: englishNameParts[1] || '',
          englishLastName: englishNameParts[2] || '',
          dateOfBirth,
          gender: gender === '1' ? 'Male' : gender === '2' ? 'Female' : gender,
          address,
          issueDate,
          expireDate,
          raw: {
            thaiFullName: thaiName,
            englishFullName: englishName
          }
        }

        // Disconnect
        reader.disconnect(reader.SCARD_LEAVE_CARD, (disconnectErr) => {
          if (disconnectErr) {
            console.error('Disconnect error:', disconnectErr.message)
          }
        })

        resolve(result)

      } catch (readErr) {
        reader.disconnect(reader.SCARD_LEAVE_CARD, () => {})
        reject(readErr)
      }
    })
  })
}

module.exports = { initCardReader, getStatus, readCard }
