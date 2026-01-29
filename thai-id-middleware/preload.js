const { contextBridge, ipcRenderer } = require('electron')

// Expose protected methods that allow the renderer process to use
// the ipcRenderer without exposing the entire object
contextBridge.exposeInMainWorld('electronAPI', {
  // Get current status
  getStatus: () => ipcRenderer.invoke('get-status'),

  // Get app version
  getVersion: () => ipcRenderer.invoke('get-version'),

  // Listen for status updates from main process
  onStatusUpdate: (callback) => {
    ipcRenderer.on('status-update', (_event, status) => callback(status))
  },

  // Remove status update listener
  removeStatusUpdateListener: () => {
    ipcRenderer.removeAllListeners('status-update')
  },

  // Read card data (for debug/test)
  readCard: () => ipcRenderer.invoke('read-card'),

  // Get debug info about the card
  debugCard: () => ipcRenderer.invoke('debug-card')
})
