const { app, BrowserWindow, Tray, Menu, ipcMain, nativeImage } = require('electron')
const path = require('path')
const { startServer, stopServer } = require('./server')
const { initCardReader, getStatus } = require('./card-reader')

let mainWindow, tray

function createWindow() {
  mainWindow = new BrowserWindow({
    width: 400,
    height: 500,
    resizable: false,
    webPreferences: {
      preload: path.join(__dirname, 'preload.js'),
      contextIsolation: true,
      nodeIntegration: false
    }
  })

  mainWindow.loadFile('index.html')

  // Hide menu bar
  mainWindow.setMenuBarVisibility(false)

  // Minimize to tray instead of closing
  mainWindow.on('close', (e) => {
    if (!app.isQuitting) {
      e.preventDefault()
      mainWindow.hide()
    }
  })
}

function createTray() {
  // Create a simple tray icon
  const iconPath = path.join(__dirname, 'icon.png')
  let trayIcon

  try {
    trayIcon = nativeImage.createFromPath(iconPath)
    if (trayIcon.isEmpty()) {
      // Create a simple colored icon if file doesn't exist
      trayIcon = nativeImage.createEmpty()
    }
  } catch {
    trayIcon = nativeImage.createEmpty()
  }

  // Resize for tray (16x16 on most platforms)
  if (!trayIcon.isEmpty()) {
    trayIcon = trayIcon.resize({ width: 16, height: 16 })
  }

  tray = new Tray(trayIcon)
  tray.setToolTip('Thai ID Middleware')

  const contextMenu = Menu.buildFromTemplate([
    {
      label: 'Show Window',
      click: () => mainWindow.show()
    },
    {
      label: 'Check Status',
      click: () => {
        const status = getStatus()
        const msg = `Server: ${status.serverRunning ? 'Running' : 'Stopped'}\nReader: ${status.readerConnected ? 'Connected' : 'Not Connected'}\nCard: ${status.cardInserted ? 'Inserted' : 'Not Inserted'}`
        console.log(msg)
        mainWindow.show()
      }
    },
    { type: 'separator' },
    {
      label: 'Quit',
      click: () => {
        app.isQuitting = true
        stopServer()
        app.quit()
      }
    }
  ])

  tray.setContextMenu(contextMenu)

  tray.on('click', () => {
    mainWindow.show()
  })

  tray.on('double-click', () => {
    mainWindow.show()
  })
}

app.whenReady().then(() => {
  createWindow()
  createTray()

  // Initialize card reader with status callback
  initCardReader((status) => {
    if (mainWindow && mainWindow.webContents) {
      mainWindow.webContents.send('status-update', status)
    }
  })

  // Start HTTP server
  startServer()

  app.on('activate', () => {
    if (BrowserWindow.getAllWindows().length === 0) {
      createWindow()
    } else {
      mainWindow.show()
    }
  })
})

app.on('window-all-closed', () => {
  if (process.platform !== 'darwin') {
    stopServer()
    app.quit()
  }
})

app.on('before-quit', () => {
  app.isQuitting = true
  stopServer()
})

// IPC handlers
ipcMain.handle('get-status', () => {
  return getStatus()
})

ipcMain.handle('get-version', () => {
  return app.getVersion()
})
