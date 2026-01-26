import '@testing-library/jest-dom'
import dotenv from 'dotenv'

// Load environment variables from .env.local for integration tests
dotenv.config({ path: '.env.local' })
