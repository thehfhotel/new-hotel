const express = require('express')
const cors = require('cors')
const { getStatus, readCard } = require('./card-reader')

const app = express()
const PORT = 9898

let server = null

// Enable CORS for all origins (localhost web apps)
app.use(cors({
  origin: ['http://localhost:3003', 'http://localhost:3000'],
  methods: ['GET', 'POST', 'OPTIONS'],
  allowedHeaders: ['Content-Type', 'Authorization']
}))

app.use(express.json())

// Health check endpoint
app.get('/health', (req, res) => {
  const status = getStatus()
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    ...status
  })
})

// Read card endpoint
app.get('/read', async (req, res) => {
  try {
    console.log('Read card request received')
    const data = await readCard()
    console.log('Card read successfully')
    res.json({
      success: true,
      data
    })
  } catch (err) {
    console.error('Read card error:', err.message)
    res.status(500).json({
      success: false,
      error: err.message
    })
  }
})

// Status endpoint (alias for health)
app.get('/status', (req, res) => {
  const status = getStatus()
  res.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    ...status
  })
})

// 404 handler
app.use((req, res) => {
  res.status(404).json({
    error: 'Not Found',
    availableEndpoints: [
      'GET /health - Check server and reader status',
      'GET /status - Alias for /health',
      'GET /read - Read Thai ID card data'
    ]
  })
})

// Error handler
app.use((err, req, res, next) => {
  console.error('Server error:', err)
  res.status(500).json({
    success: false,
    error: 'Internal server error'
  })
})

function startServer() {
  return new Promise((resolve, reject) => {
    try {
      server = app.listen(PORT, '127.0.0.1', () => {
        console.log(`Thai ID Middleware server running on http://localhost:${PORT}`)
        resolve(server)
      })

      server.on('error', (err) => {
        if (err.code === 'EADDRINUSE') {
          console.error(`Port ${PORT} is already in use`)
        }
        reject(err)
      })
    } catch (err) {
      reject(err)
    }
  })
}

function stopServer() {
  return new Promise((resolve) => {
    if (server) {
      server.close(() => {
        console.log('Server stopped')
        server = null
        resolve()
      })
    } else {
      resolve()
    }
  })
}

module.exports = { startServer, stopServer }
