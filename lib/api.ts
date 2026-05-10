/**
 * Thin fetch wrapper that always includes the session cookie and surfaces
 * 401s as a typed error so callers (notably AuthContext) can react without
 * having to inspect raw `Response` objects.
 *
 * Scope (PR3): used by the auth flow only. The rest of the app keeps its
 * existing direct `fetch` / `useBranchFetch` calls — refactoring all of them
 * is out of scope for this PR.
 */

export class ApiError extends Error {
  constructor(
    message: string,
    public readonly status: number,
    public readonly body: unknown,
  ) {
    super(message)
    this.name = 'ApiError'
  }
}

interface ApiFetchOptions extends Omit<RequestInit, 'body'> {
  /** JSON-serializable body. Stringified + Content-Type set automatically. */
  json?: unknown
}

/**
 * Fetch helper that:
 *  - always sends the HttpOnly session cookie via `credentials: 'include'`
 *  - JSON-encodes the request body when `json` is provided
 *  - parses the response body as JSON (best effort) before returning
 *  - throws `ApiError` on non-2xx responses so callers can `catch (e)` once
 */
export async function apiFetch<T = unknown>(
  path: string,
  options: ApiFetchOptions = {},
): Promise<T> {
  const { json, headers, ...rest } = options
  const init: RequestInit = {
    credentials: 'include',
    ...rest,
    headers: {
      ...(json !== undefined ? { 'Content-Type': 'application/json' } : {}),
      ...headers,
    },
  }
  if (json !== undefined) {
    init.body = JSON.stringify(json)
  }

  const response = await fetch(path, init)
  const parsedBody = await parseJsonSafely(response)

  if (!response.ok) {
    const message = extractErrorMessage(parsedBody) ?? response.statusText
    throw new ApiError(message, response.status, parsedBody)
  }

  return parsedBody as T
}

async function parseJsonSafely(response: Response): Promise<unknown> {
  const text = await response.text()
  if (text.length === 0) return null
  try {
    return JSON.parse(text)
  } catch {
    return text
  }
}

function extractErrorMessage(body: unknown): string | null {
  if (body && typeof body === 'object' && 'error' in body) {
    const value = (body as { error: unknown }).error
    if (typeof value === 'string') return value
  }
  return null
}
