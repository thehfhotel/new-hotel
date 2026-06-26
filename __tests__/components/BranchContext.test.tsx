/**
 * @jest-environment jsdom
 */

/**
 * HF Ville availability — regression tests.
 *
 * "HF Ville is not loading" reduces to: the UI disables the HF Ville branch
 * whenever `villeAvailable` is false. `villeAvailable` is derived in
 * BranchContext from GET /api/mode (`data.villeAvailable`, camelCase). These
 * tests pin that contract and the gating consequence so the failure mode can't
 * silently return.
 */

import { render, screen, waitFor, fireEvent, act } from '@testing-library/react'
import { BranchProvider, useBranch, BRANCH_LABELS } from '@/contexts/BranchContext'

function modeResponse(body: unknown, ok = true): Response {
  return {
    ok,
    status: ok ? 200 : 500,
    json: () => Promise.resolve(body),
    text: () => Promise.resolve(JSON.stringify(body)),
  } as unknown as Response
}

/** Non-JSON error body (what the prod edge / a down backend returns) — json()
 *  rejects, exercising BranchContext's catch path. */
function brokenModeResponse(): Response {
  return {
    ok: false,
    status: 500,
    json: () => Promise.reject(new SyntaxError("Unexpected token 'I', \"Internal S\"... is not valid JSON")),
    text: () => Promise.resolve('Internal Server Error'),
  } as unknown as Response
}

// Mirrors exactly how the classic Sidebar and the new V2Shell gate the control:
//   disabled = b === 'hfville' && !villeAvailable
function BranchConsumer() {
  const { branch, setBranch, villeAvailable, villeWritesEnabled, canWrite } = useBranch()
  return (
    <div>
      <div data-testid="branch">{branch}</div>
      <div data-testid="ville-available">{String(villeAvailable)}</div>
      <div data-testid="ville-writes">{String(villeWritesEnabled)}</div>
      <div data-testid="can-write">{String(canWrite)}</div>
      <button data-testid="ville-btn" disabled={!villeAvailable} onClick={() => setBranch('hfville')}>
        {BRANCH_LABELS.hfville}
      </button>
      <button data-testid="hotel-btn" onClick={() => setBranch('hfhotel')}>
        {BRANCH_LABELS.hfhotel}
      </button>
    </div>
  )
}

describe('BranchContext — HF Ville availability gating', () => {
  let fetchMock: jest.Mock

  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    localStorage.clear()
  })

  test('enables HF Ville when /api/mode reports villeAvailable:true', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true }))

    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(screen.getByTestId('ville-available')).toHaveTextContent('true'))
    expect(screen.getByTestId('ville-btn')).not.toBeDisabled()
    expect(fetchMock).toHaveBeenCalledWith('/api/mode')
  })

  test('disables HF Ville when villeAvailable:false (the "not loading" condition)', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: false }))

    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    expect(screen.getByTestId('ville-available')).toHaveTextContent('false')
    expect(screen.getByTestId('ville-btn')).toBeDisabled()
  })

  test('requires camelCase villeAvailable — snake_case ville_available must NOT enable Ville', async () => {
    // Guards the FE<->BE contract: the Rust ModeResponse is #[serde(rename_all="camelCase")].
    // If it ever regressed to snake_case, HF Ville would silently stay disabled.
    fetchMock.mockResolvedValue(modeResponse({ success: true, ville_available: true }))

    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    expect(screen.getByTestId('ville-available')).toHaveTextContent('false')
    expect(screen.getByTestId('ville-btn')).toBeDisabled()
  })

  test('stays unavailable (and does not crash) when /api/mode fails', async () => {
    fetchMock.mockResolvedValue(brokenModeResponse())

    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    // Give the rejected json() a tick to settle through the catch.
    await act(async () => {
      await Promise.resolve()
    })
    expect(screen.getByTestId('ville-available')).toHaveTextContent('false')
    expect(screen.getByTestId('ville-btn')).toBeDisabled()
  })

  test('does not enable Ville when success is false even if villeAvailable is true', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: false, villeAvailable: true }))

    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    expect(screen.getByTestId('ville-available')).toHaveTextContent('false')
  })

  test('persists the selected branch to localStorage and restores it on mount', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true }))

    const { unmount } = render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )

    await waitFor(() => expect(screen.getByTestId('ville-btn')).not.toBeDisabled())
    act(() => {
      fireEvent.click(screen.getByTestId('ville-btn'))
    })
    expect(localStorage.getItem('selected-branch')).toBe('hfville')
    expect(screen.getByTestId('branch')).toHaveTextContent('hfville')

    unmount()
    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )
    await waitFor(() => expect(screen.getByTestId('branch')).toHaveTextContent('hfville'))
  })
})

describe('BranchContext — HF Ville WRITE gating (canWrite / villeWritesEnabled)', () => {
  let fetchMock: jest.Mock

  beforeEach(() => {
    fetchMock = jest.fn()
    global.fetch = fetchMock as unknown as typeof fetch
    localStorage.clear()
  })

  test('HF Hotel is always writable regardless of villeWritesEnabled', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true, villeWritesEnabled: false }))
    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )
    await waitFor(() => expect(fetchMock).toHaveBeenCalledWith('/api/mode'))
    expect(screen.getByTestId('branch')).toHaveTextContent('hfhotel')
    expect(screen.getByTestId('can-write')).toHaveTextContent('true')
  })

  test('HF Ville is NOT writable when villeWritesEnabled:false (view-only)', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true, villeWritesEnabled: false }))
    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )
    await waitFor(() => expect(screen.getByTestId('ville-btn')).not.toBeDisabled())
    act(() => {
      fireEvent.click(screen.getByTestId('ville-btn'))
    })
    expect(screen.getByTestId('branch')).toHaveTextContent('hfville')
    expect(screen.getByTestId('ville-writes')).toHaveTextContent('false')
    expect(screen.getByTestId('can-write')).toHaveTextContent('false')
  })

  test('HF Ville becomes writable when villeWritesEnabled:true', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true, villeWritesEnabled: true }))
    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )
    await waitFor(() => expect(screen.getByTestId('ville-writes')).toHaveTextContent('true'))
    act(() => {
      fireEvent.click(screen.getByTestId('ville-btn'))
    })
    expect(screen.getByTestId('branch')).toHaveTextContent('hfville')
    expect(screen.getByTestId('can-write')).toHaveTextContent('true')
  })

  test('villeWritesEnabled must be camelCase — snake_case must not enable writes', async () => {
    fetchMock.mockResolvedValue(modeResponse({ success: true, villeAvailable: true, ville_writes_enabled: true }))
    render(
      <BranchProvider>
        <BranchConsumer />
      </BranchProvider>,
    )
    // Wait for villeAvailable to resolve so the branch button is clickable;
    // villeWritesEnabled must stay false (snake_case ignored).
    await waitFor(() => expect(screen.getByTestId('ville-btn')).not.toBeDisabled())
    expect(screen.getByTestId('ville-writes')).toHaveTextContent('false')
    act(() => {
      fireEvent.click(screen.getByTestId('ville-btn'))
    })
    expect(screen.getByTestId('branch')).toHaveTextContent('hfville')
    expect(screen.getByTestId('can-write')).toHaveTextContent('false')
  })
})
