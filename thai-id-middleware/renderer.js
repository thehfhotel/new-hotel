// DOM elements
const serverStatus = document.getElementById('server-status')
const readerStatus = document.getElementById('reader-status')
const cardStatus = document.getElementById('card-status')
const readerName = document.getElementById('reader-name')
const versionEl = document.getElementById('version')

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

// Start
init()
