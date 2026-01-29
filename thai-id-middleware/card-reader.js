let pcsc = null
let reader = null
let cardInserted = false
let readerConnected = false
let statusCallback = () => {}
let currentProtocol = null
let pcscliteAvailable = false
let cardATR = null // Store ATR when card is inserted

// Operation counter for diagnostic logging
let operationCounter = 0
function getOpId() {
  return `[op:${++operationCounter}]`
}

function getProtocolName(protocol, r) {
  if (!r) return `Unknown(${protocol})`
  return protocol === r.SCARD_PROTOCOL_T0 ? 'T=0' :
         protocol === r.SCARD_PROTOCOL_T1 ? 'T=1' : `Unknown(${protocol})`
}

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
            cardATR = null
            console.log('Card removed')
            statusCallback(getStatus())
          } else if ((changes & r.SCARD_STATE_PRESENT) && (status.state & r.SCARD_STATE_PRESENT)) {
            // Card inserted - capture ATR
            cardInserted = true
            if (status.atr && status.atr.length > 0) {
              cardATR = status.atr.toString('hex').toUpperCase()
              console.log('Card inserted, ATR:', cardATR)
            } else {
              console.log('Card inserted')
            }
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

// GET RESPONSE command for retrieving pending data (ISO 7816-4)
async function getResponse(protocol, length) {
  const cmd = Buffer.from([0x00, 0xC0, 0x00, 0x00, length])
  return await transmit(protocol, cmd)
}

// Smart transmit wrapper that handles SW1=61 (more data available)
// SW1=61 means command succeeded and SW2 bytes of response data are available
async function transmitWithGetResponse(protocol, command) {
  let response = await transmit(protocol, command)
  let sw1 = response[response.length - 2]
  let sw2 = response[response.length - 1]

  // Handle SW1=61 (more data available) by sending GET RESPONSE
  while (sw1 === 0x61) {
    const moreData = await getResponse(protocol, sw2)
    // Combine data (excluding old SW) with new response
    response = Buffer.concat([response.slice(0, -2), moreData])
    sw1 = response[response.length - 2]
    sw2 = response[response.length - 1]
  }

  return response
}

// Reset card by connecting briefly and disconnecting with SCARD_RESET_CARD
// This clears the card's internal state and prepares it for the next connection
async function resetCard() {
  const opId = getOpId()
  console.log(`${opId} resetCard: attempting connect for reset...`)

  return new Promise((resolve) => {
    // Try to connect briefly just to reset
    reader.connect({ share_mode: reader.SCARD_SHARE_SHARED }, (err, protocol) => {
      if (!err) {
        const protoName = getProtocolName(protocol, reader)
        console.log(`${opId} resetCard: connected with ${protoName}, disconnecting with RESET...`)

        reader.disconnect(reader.SCARD_RESET_CARD, (disconnectErr) => {
          if (disconnectErr) {
            console.log(`${opId} resetCard: disconnect error: ${disconnectErr.message}`)
          } else {
            console.log(`${opId} resetCard: disconnect completed successfully`)
          }
          resolve(true)
        })
      } else {
        console.log(`${opId} resetCard: connect failed: ${err.message}`)
        resolve(false)
      }
    })
  })
}

// Retry connect with delay for cards that need time to initialize after insertion
// This handles the race condition when app starts with card already inserted
// Uses 5 retries with 1000ms delay (5 seconds total) for cold-inserted cards
async function connectWithRetry(maxRetries = 5, delayMs = 1000) {
  const opId = getOpId()
  console.log(`${opId} connectWithRetry: starting (maxRetries=${maxRetries}, delayMs=${delayMs})`)

  return new Promise((resolve, reject) => {
    let attempts = 0
    let resetAttempted = false

    async function tryConnect() {
      attempts++
      console.log(`${opId} connectWithRetry: attempt ${attempts}/${maxRetries}...`)

      reader.connect({ share_mode: reader.SCARD_SHARE_SHARED }, async (err, protocol) => {
        if (err) {
          console.log(`${opId} connectWithRetry: attempt ${attempts} failed: ${err.message}`)

          // Retry on any error (not just 'unresponsive') if we have retries left
          if (attempts < maxRetries) {
            // On first failure, try a reset to clear any stale state
            if (!resetAttempted) {
              resetAttempted = true
              console.log(`${opId} connectWithRetry: attempting pre-connect reset...`)
              const resetResult = await resetCard()
              console.log(`${opId} connectWithRetry: reset returned ${resetResult}`)
            }
            console.log(`${opId} connectWithRetry: retrying in ${delayMs}ms...`)
            setTimeout(tryConnect, delayMs)
          } else {
            console.log(`${opId} connectWithRetry: all attempts exhausted`)
            reject(new Error(`Connection failed after ${maxRetries} attempts: ${err.message}`))
          }
        } else {
          const protoName = getProtocolName(protocol, reader)
          console.log(`${opId} connectWithRetry: SUCCESS with ${protoName}`)
          resolve(protocol)
        }
      })
    }

    tryConnect()
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
  const opId = getOpId()
  console.log(`${opId} readCard: starting...`)

  if (!reader) {
    console.log(`${opId} readCard: FAILED - no reader connected`)
    throw new Error('No card reader connected')
  }

  if (!cardInserted) {
    console.log(`${opId} readCard: FAILED - no card inserted`)
    throw new Error('No card inserted')
  }

  let protocol
  try {
    protocol = await connectWithRetry()
  } catch (connectErr) {
    console.log(`${opId} readCard: connect failed: ${connectErr.message}`)
    throw connectErr
  }

  currentProtocol = protocol
  console.log(`${opId} readCard: connected, selecting applet...`)

  try {
    // Select Thai ID applet (use transmitWithGetResponse to handle SW 61XX)
    const selectResponse = await transmitWithGetResponse(protocol, APDU.SELECT_APPLET)
    const sw = selectResponse.slice(-2)
    if (sw[0] !== 0x90 || sw[1] !== 0x00) {
      throw new Error('Failed to select Thai ID applet. Is this a Thai National ID card?')
    }
    console.log(`${opId} readCard: applet selected, reading data...`)

    // Read all data sequentially (smart cards don't support parallel operations)
    // Use transmitWithGetResponse to properly handle SW1=61 (more data available)
    const cidResp = await transmitWithGetResponse(protocol, APDU.READ_CID)
    const thNameResp = await transmitWithGetResponse(protocol, APDU.READ_TH_FULLNAME)
    const enNameResp = await transmitWithGetResponse(protocol, APDU.READ_EN_FULLNAME)
    const dobResp = await transmitWithGetResponse(protocol, APDU.READ_DOB)
    const genderResp = await transmitWithGetResponse(protocol, APDU.READ_GENDER)
    const addrResp = await transmitWithGetResponse(protocol, APDU.READ_ADDRESS)
    const issueDateResp = await transmitWithGetResponse(protocol, APDU.READ_ISSUE_DATE)
    const expireDateResp = await transmitWithGetResponse(protocol, APDU.READ_EXPIRE_DATE)

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

    console.log(`${opId} readCard: SUCCESS, disconnecting...`)
    // Disconnect with SCARD_RESET_CARD to clear card state for next read
    reader.disconnect(reader.SCARD_RESET_CARD, (disconnectErr) => {
      if (disconnectErr) {
        console.log(`${opId} readCard: disconnect error: ${disconnectErr.message}`)
      } else {
        console.log(`${opId} readCard: disconnect completed`)
      }
    })

    return result

  } catch (readErr) {
    console.log(`${opId} readCard: FAILED: ${readErr.message}`)
    reader.disconnect(reader.SCARD_RESET_CARD, (disconnectErr) => {
      if (disconnectErr) {
        console.log(`${opId} readCard: cleanup disconnect error: ${disconnectErr.message}`)
      }
    })
    throw readErr
  }
}

// Known Thai ID Card Application IDs to try
const KNOWN_AIDS = [
  { name: 'TH-NID Standard', aid: [0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x01] },
  { name: 'TH-NID Alternate', aid: [0xA0, 0x00, 0x00, 0x00, 0x54, 0x48, 0x00, 0x02] },
  { name: 'TH-MOI', aid: [0xA0, 0x00, 0x00, 0x00, 0x84, 0x54, 0x48, 0x00, 0x01] },
  { name: 'EMV Payment', aid: [0xA0, 0x00, 0x00, 0x00, 0x04, 0x10, 0x10] },
]

async function debugCard() {
  const opId = getOpId()
  console.log(`${opId} debugCard: starting...`)

  if (!reader) {
    console.log(`${opId} debugCard: FAILED - no reader connected`)
    return { error: 'No card reader connected', atr: null, aidResults: [] }
  }

  if (!cardInserted) {
    console.log(`${opId} debugCard: FAILED - no card inserted`)
    return { error: 'No card inserted', atr: cardATR, aidResults: [] }
  }

  let protocol
  try {
    protocol = await connectWithRetry()
  } catch (err) {
    console.log(`${opId} debugCard: connect failed: ${err.message}`)
    return {
      error: err.message,
      atr: cardATR,
      aidResults: [],
      protocol: null
    }
  }

  const protocolName = getProtocolName(protocol, reader)
  console.log(`${opId} debugCard: connected with ${protocolName}, testing AIDs...`)

  const aidResults = []

  // Try each known AID
  for (const aidInfo of KNOWN_AIDS) {
    try {
      // Build SELECT APDU: 00 A4 04 00 [length] [AID bytes]
      const selectCmd = Buffer.from([
        0x00, 0xA4, 0x04, 0x00,
        aidInfo.aid.length,
        ...aidInfo.aid
      ])

      const response = await transmitWithGetResponse(protocol, selectCmd)
      const sw1 = response[response.length - 2]
      const sw2 = response[response.length - 1]
      const statusWord = ((sw1 << 8) | sw2).toString(16).toUpperCase().padStart(4, '0')

      // SW1=90 (success) OR SW1=61 (success with more data available)
      const success = sw1 === 0x90 || sw1 === 0x61
      const responseHex = response.toString('hex').toUpperCase()

      console.log(`${opId} debugCard: AID ${aidInfo.name} -> SW=${statusWord} (${success ? 'SUCCESS' : 'FAILED'})`)

      aidResults.push({
        name: aidInfo.name,
        aid: Buffer.from(aidInfo.aid).toString('hex').toUpperCase(),
        success,
        statusWord,
        response: responseHex,
        description: getStatusDescription(sw1, sw2)
      })
    } catch (aidErr) {
      console.log(`${opId} debugCard: AID ${aidInfo.name} -> ERROR: ${aidErr.message}`)
      aidResults.push({
        name: aidInfo.name,
        aid: Buffer.from(aidInfo.aid).toString('hex').toUpperCase(),
        success: false,
        error: aidErr.message
      })
    }
  }

  // Try to read raw data from the first successful AID
  let rawReadResult = null
  const successfulAid = aidResults.find(r => r.success)
  if (successfulAid) {
    try {
      // Try reading CID as a test
      const cidResponse = await transmitWithGetResponse(protocol, APDU.READ_CID)
      rawReadResult = {
        command: 'READ_CID',
        commandHex: APDU.READ_CID.toString('hex').toUpperCase(),
        response: cidResponse.toString('hex').toUpperCase(),
        sw1: cidResponse[cidResponse.length - 2],
        sw2: cidResponse[cidResponse.length - 1]
      }
      console.log(`${opId} debugCard: READ_CID succeeded`)
    } catch (readErr) {
      console.log(`${opId} debugCard: READ_CID failed: ${readErr.message}`)
      rawReadResult = { error: readErr.message }
    }
  }

  console.log(`${opId} debugCard: SUCCESS, disconnecting...`)
  // Disconnect with SCARD_RESET_CARD to clear card state for next read
  reader.disconnect(reader.SCARD_RESET_CARD, (disconnectErr) => {
    if (disconnectErr) {
      console.log(`${opId} debugCard: disconnect error: ${disconnectErr.message}`)
    } else {
      console.log(`${opId} debugCard: disconnect completed`)
    }
  })

  return {
    atr: cardATR,
    protocol: protocolName,
    readerName: reader.name,
    aidResults,
    rawReadResult,
    timestamp: new Date().toISOString()
  }
}

function getStatusDescription(sw1, sw2) {
  if (sw1 === 0x90 && sw2 === 0x00) return 'Success'
  if (sw1 === 0x6A && sw2 === 0x82) return 'File/Application not found'
  if (sw1 === 0x6A && sw2 === 0x86) return 'Incorrect P1/P2 parameters'
  if (sw1 === 0x6A && sw2 === 0x81) return 'Function not supported'
  if (sw1 === 0x69 && sw2 === 0x85) return 'Conditions not satisfied'
  if (sw1 === 0x6E && sw2 === 0x00) return 'Class not supported'
  if (sw1 === 0x6D && sw2 === 0x00) return 'Instruction not supported'
  if (sw1 === 0x67 && sw2 === 0x00) return 'Wrong length'
  if (sw1 === 0x6C) return `Wrong Le, expected ${sw2} bytes`
  if (sw1 === 0x61) return `${sw2} bytes available`
  return 'Unknown status'
}

module.exports = { initCardReader, getStatus, readCard, debugCard }
