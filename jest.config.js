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
  // Ignore git worktrees (created by agent isolation) so jest doesn't double-run tests.
  // Anchored to <rootDir> so that running jest FROM inside a worktree only excludes
  // *child* worktrees, not the worktree itself. (The unrooted previous pattern
  // matched any path containing `/.claude/worktrees/` and zeroed out the
  // worktree's own tests, which broke the husky pre-push hook.)
  testPathIgnorePatterns: ['/node_modules/', '<rootDir>/\\.claude/worktrees/'],
  modulePathIgnorePatterns: ['<rootDir>/\\.claude/worktrees/'],
  transformIgnorePatterns: [
    '/node_modules/(?!(@azure|tedious)/)',
  ],
  // Longer timeout for integration tests that spin up servers
  testTimeout: 30000,
}

module.exports = createJestConfig(customJestConfig)
