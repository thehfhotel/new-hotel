'use client'

import { useBranch } from '@/contexts/BranchContext'
import { useCallback } from 'react'

/**
 * Returns a fetch function that automatically appends ?branch=X to API calls.
 * Only appends to /api/* URLs. Non-API URLs are passed through unchanged.
 */
export function useBranchFetch() {
  const { branch } = useBranch()

  const branchFetch = useCallback(
    (input: string | URL | Request, init?: RequestInit) => {
      if (typeof input === 'string' && input.startsWith('/api/')) {
        const separator = input.includes('?') ? '&' : '?'
        return fetch(`${input}${separator}branch=${branch}`, init)
      }
      return fetch(input, init)
    },
    [branch]
  )

  return branchFetch
}
