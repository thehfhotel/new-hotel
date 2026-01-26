const nextJest = require('next/jest')

const createJestConfig = nextJest({
  dir: './',
})

const customJestConfig = {
  setupFilesAfterEnv: ['<rootDir>/jest.setup.js'],
  testEnvironment: 'node', // Use node for API tests
  moduleNameMapper: {
    '^@/(.*)$': '<rootDir>/$1',
  },
  testMatch: ['**/__tests__/**/*.test.ts', '**/__tests__/**/*.test.tsx'],
  transformIgnorePatterns: [
    '/node_modules/(?!(@azure|tedious)/)',
  ],
  // Longer timeout for integration tests that spin up servers
  testTimeout: 30000,
}

module.exports = createJestConfig(customJestConfig)
