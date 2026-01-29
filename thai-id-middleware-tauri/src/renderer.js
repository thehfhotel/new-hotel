// Tauri API
const { invoke } = window.__TAURI__.core;
const { listen } = window.__TAURI__.event;
const { getCurrentWindow } = window.__TAURI__.window;

// Window sizes
const WINDOW_WIDTH = 400
const WINDOW_HEIGHT_COMPACT = 340
const WINDOW_HEIGHT_DEBUG = 760

// DOM elements
const serverStatus = document.getElementById('server-status')
const readerStatus = document.getElementById('reader-status')
const cardStatus = document.getElementById('card-status')
const versionEl = document.getElementById('version')
const debugOutput = document.getElementById('debug-output')
const btnTestRead = document.getElementById('btn-test-read')
const btnDebug = document.getElementById('btn-debug')
const btnClear = document.getElementById('btn-clear')
const btnToggleDebug = document.getElementById('btn-toggle-debug')

// Debug mode state
let debugMode = false

// Update UI based on status
function updateUI(status) {
  // Server status (always running in Tauri)
  serverStatus.textContent = 'Running :9898'
  serverStatus.className = 'badge green'

  // Reader status
  if (status.readerConnected) {
    readerStatus.textContent = 'Connected'
    readerStatus.className = 'badge green'
  } else {
    readerStatus.textContent = 'Not Connected'
    readerStatus.className = 'badge gray'
  }

  // Card status
  if (status.cardInserted) {
    cardStatus.textContent = 'Inserted'
    cardStatus.className = 'badge green'
  } else if (status.readerConnected) {
    cardStatus.textContent = 'Not Inserted'
    cardStatus.className = 'badge yellow'
  } else {
    cardStatus.textContent = 'Not Inserted'
    cardStatus.className = 'badge gray'
  }

}

// Initialize
async function init() {
  // Get version
  try {
    const version = await invoke('get_version')
    versionEl.textContent = `v${version}`
  } catch (err) {
    console.error('Failed to get version:', err)
  }

  // Get initial status
  try {
    const status = await invoke('get_status')
    updateUI(status)
  } catch (err) {
    console.error('Failed to get initial status:', err)
  }

  // Listen for status updates from tray menu
  try {
    await listen('check-status', async () => {
      const status = await invoke('get_status')
      updateUI(status)
    })
  } catch (err) {
    console.error('Failed to listen for status updates:', err)
  }

  // Poll status every 2 seconds
  setInterval(async () => {
    try {
      const status = await invoke('get_status')
      updateUI(status)
    } catch (err) {
      console.error('Failed to get status:', err)
    }
  }, 2000)
}

// Debug functions
function appendOutput(text, type = 'info') {
  const line = document.createElement('div')
  line.className = `output-line ${type}`
  line.textContent = text
  debugOutput.appendChild(line)
  debugOutput.scrollTop = debugOutput.scrollHeight
}

function clearOutput() {
  debugOutput.innerHTML = ''
}

function formatCardData(data) {
  // Show photo if available
  if (data.photo) {
    const photoContainer = document.createElement('div')
    photoContainer.className = 'photo-container'
    const img = document.createElement('img')
    img.src = `data:image/jpeg;base64,${data.photo}`
    img.alt = 'ID Photo'
    img.className = 'id-photo'
    photoContainer.appendChild(img)
    debugOutput.appendChild(photoContainer)
  }

  appendOutput('=== Card Data ===', 'header')
  appendOutput(`CID: ${data.cid}`)
  appendOutput(`Thai Name: ${data.raw.thaiFullName}`)
  appendOutput(`English Name: ${data.raw.englishFullName}`)
  appendOutput(`DOB: ${data.dateOfBirth}`)
  appendOutput(`Gender: ${data.gender}`)
  appendOutput(`Address: ${data.address}`)
  appendOutput(`Issue: ${data.issueDate}`)
  appendOutput(`Expire: ${data.expireDate}`)
}

function formatDebugInfo(info) {
  appendOutput('=== Debug Info ===', 'header')
  appendOutput(`Timestamp: ${info.timestamp || new Date().toISOString()}`)

  if (info.error) {
    appendOutput(`Error: ${info.error}`, 'error')
    return
  }

  appendOutput(`ATR: ${info.atr || 'N/A'}`)
  appendOutput(`Protocol: ${info.protocol || 'N/A'}`)
  appendOutput(`Reader: ${info.readerName || 'N/A'}`)

  if (info.aidResults && info.aidResults.length > 0) {
    appendOutput('')
    appendOutput('=== AID Test Results ===', 'header')
    for (const aid of info.aidResults) {
      const status = aid.success ? 'OK' : 'FAIL'
      const statusClass = aid.success ? 'success' : 'error'
      appendOutput(`[${status}] ${aid.name}`, statusClass)
      appendOutput(`  AID: ${aid.aid}`)
      if (aid.statusWord) {
        appendOutput(`  SW: ${aid.statusWord} (${aid.description})`)
      }
      if (aid.error) {
        appendOutput(`  Error: ${aid.error}`, 'error')
      }
    }
  }

  if (info.rawReadResult) {
    appendOutput('')
    appendOutput('=== Raw Read Test ===', 'header')
    if (info.rawReadResult.error) {
      appendOutput(`Error: ${info.rawReadResult.error}`, 'error')
    } else {
      appendOutput(`Command: ${info.rawReadResult.command}`)
      appendOutput(`APDU: ${info.rawReadResult.commandHex}`)
      appendOutput(`Response: ${info.rawReadResult.response}`)
      appendOutput(`SW: ${info.rawReadResult.sw1.toString(16).padStart(2, '0')}${info.rawReadResult.sw2.toString(16).padStart(2, '0')}`)
    }
  }
}

// Debug button handlers
btnTestRead.addEventListener('click', async () => {
  clearOutput()
  appendOutput('Reading card...', 'info')
  btnTestRead.disabled = true

  try {
    const result = await invoke('read_card', { includePhoto: true })
    clearOutput()

    if (result.success) {
      appendOutput('Card read successful!', 'success')
      formatCardData(result.data)
    } else {
      appendOutput('Card read failed!', 'error')
      appendOutput(`Error: ${result.error}`, 'error')
    }
  } catch (err) {
    clearOutput()
    appendOutput('Card read error!', 'error')
    appendOutput(`Exception: ${err}`, 'error')
  } finally {
    btnTestRead.disabled = false
  }
})

btnDebug.addEventListener('click', async () => {
  clearOutput()
  appendOutput('Getting debug info...', 'info')
  btnDebug.disabled = true

  try {
    const result = await invoke('debug_card')
    clearOutput()
    formatDebugInfo(result)
  } catch (err) {
    clearOutput()
    appendOutput('Debug error!', 'error')
    appendOutput(`Exception: ${err}`, 'error')
  } finally {
    btnDebug.disabled = false
  }
})

btnClear.addEventListener('click', () => {
  clearOutput()
})

// Debug mode toggle
async function setDebugMode(enabled) {
  debugMode = enabled
  const debugElements = document.querySelectorAll('.debug-only')
  debugElements.forEach(el => {
    el.style.display = enabled ? '' : 'none'
  })
  btnToggleDebug.classList.toggle('active', enabled)

  // Resize window
  try {
    const appWindow = getCurrentWindow()
    const newHeight = enabled ? WINDOW_HEIGHT_DEBUG : WINDOW_HEIGHT_COMPACT
    const { LogicalSize } = window.__TAURI__.window
    await appWindow.setSize(new LogicalSize(WINDOW_WIDTH, newHeight))
  } catch (err) {
    console.error('Failed to resize window:', err)
  }
}

btnToggleDebug.addEventListener('click', () => {
  setDebugMode(!debugMode)
})

// Start with debug mode off
setDebugMode(false)

// Start
init()
