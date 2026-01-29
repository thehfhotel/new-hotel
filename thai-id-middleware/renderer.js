// DOM elements
const serverStatus = document.getElementById('server-status')
const readerStatus = document.getElementById('reader-status')
const cardStatus = document.getElementById('card-status')
const readerName = document.getElementById('reader-name')
const versionEl = document.getElementById('version')
const debugOutput = document.getElementById('debug-output')
const btnTestRead = document.getElementById('btn-test-read')
const btnDebug = document.getElementById('btn-debug')
const btnClear = document.getElementById('btn-clear')

// Update UI based on status
function updateUI(status) {
  // Server status
  if (status.serverRunning) {
    serverStatus.textContent = `Running :${status.port}`
    serverStatus.className = 'badge green'
  } else {
    serverStatus.textContent = 'Stopped'
    serverStatus.className = 'badge red'
  }

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

  // Reader name
  if (status.readerName) {
    readerName.textContent = `Reader: ${status.readerName}`
    readerName.style.display = 'block'
  } else {
    readerName.style.display = 'none'
  }
}

// Initialize
async function init() {
  // Get version
  try {
    const version = await window.electronAPI.getVersion()
    versionEl.textContent = `v${version}`
  } catch (err) {
    console.error('Failed to get version:', err)
  }

  // Get initial status
  try {
    const status = await window.electronAPI.getStatus()
    updateUI(status)
  } catch (err) {
    console.error('Failed to get initial status:', err)
  }

  // Listen for status updates
  window.electronAPI.onStatusUpdate((status) => {
    updateUI(status)
  })
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
    const result = await window.electronAPI.readCard()
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
    appendOutput(`Exception: ${err.message}`, 'error')
  } finally {
    btnTestRead.disabled = false
  }
})

btnDebug.addEventListener('click', async () => {
  clearOutput()
  appendOutput('Getting debug info...', 'info')
  btnDebug.disabled = true

  try {
    const result = await window.electronAPI.debugCard()
    clearOutput()
    formatDebugInfo(result)
  } catch (err) {
    clearOutput()
    appendOutput('Debug error!', 'error')
    appendOutput(`Exception: ${err.message}`, 'error')
  } finally {
    btnDebug.disabled = false
  }
})

btnClear.addEventListener('click', () => {
  clearOutput()
})

// Start
init()
