/**
 * @jest-environment jsdom
 */

/**
 * The `/hk` branch picker's three shapes — and specifically the EMPTY one
 * introduced by per-employee location enforcement (wave-4 §C).
 *
 * `GET /api/hk/me` can now legitimately answer `branches: []`: HF ID has no
 * location on file for this maid, her property is not served by this
 * deployment yet, or the lookup could not answer. The picker must NEVER render
 * a chooser with zero choices (a dead end that reads as a bug), and must never
 * invent a branch to tap — offering a guess is the exact wrong-property bug
 * location enforcement exists to close.
 */

import { render, screen } from '@testing-library/react'
import {
  HkBranchChip,
  HkBranchesUnavailable,
  HkBranchPicker,
} from '@/app/hk/HkBranchPicker'
import { branchesUnavailableMessage, type HkBranchOption } from '@/app/hk/hk-lib'

const HOTEL: HkBranchOption = { id: 'hfhotel', labelTh: 'ฮาร์เบอร์ฟร้อนท์' }
const VILLE: HkBranchOption = { id: 'hfville', labelTh: 'วิลล์' }

describe('HkBranchesUnavailable', () => {
  it('renders the no_location message and NO branch buttons', () => {
    render(<HkBranchesUnavailable reason="no_location" />)

    expect(
      screen.getByText(branchesUnavailableMessage('no_location'))
    ).toBeInTheDocument()
    // The load-bearing assertion: nothing tappable. A maid with no resolved
    // location must not be handed a branch to pick.
    expect(screen.queryAllByRole('button')).toHaveLength(0)
    expect(screen.queryByText(HOTEL.labelTh)).not.toBeInTheDocument()
    expect(screen.queryByText(VILLE.labelTh)).not.toBeInTheDocument()
  })

  it('renders the RETRY message for lookup_unavailable, not the admin one', () => {
    render(<HkBranchesUnavailable reason="lookup_unavailable" />)

    expect(
      screen.getByText(branchesUnavailableMessage('lookup_unavailable'))
    ).toBeInTheDocument()
    expect(
      screen.queryByText(branchesUnavailableMessage('no_location'))
    ).not.toBeInTheDocument()
    expect(screen.queryAllByRole('button')).toHaveLength(0)
  })

  it('still shows an actionable message when the reason is null', () => {
    // A backend that answered `branches: []` with no reason (or a reason this
    // bundle predates) must still produce real copy, never an empty panel.
    render(<HkBranchesUnavailable reason={null} />)

    expect(screen.getByRole('alert')).toHaveTextContent(
      branchesUnavailableMessage(null)
    )
    expect(screen.queryAllByRole('button')).toHaveLength(0)
  })

  it('is announced as an alert so it is not read as a silent empty state', () => {
    render(<HkBranchesUnavailable reason="no_location" />)
    expect(screen.getByRole('alert')).toBeInTheDocument()
  })
})

describe('HkBranchPicker (unchanged behaviour)', () => {
  it('renders one button per offered branch', () => {
    render(<HkBranchPicker branches={[HOTEL, VILLE]} onPick={jest.fn()} />)

    expect(screen.getByText(HOTEL.labelTh)).toBeInTheDocument()
    expect(screen.getByText(VILLE.labelTh)).toBeInTheDocument()
    expect(screen.queryAllByRole('button')).toHaveLength(2)
  })

  it('renders the single-branch case too — the enforced-maid shape', () => {
    // With enforcement ON the intersection is normally exactly one branch, and
    // /hk auto-selects it without rendering this component at all. If it IS
    // rendered, it must still offer only that one.
    render(<HkBranchPicker branches={[VILLE]} onPick={jest.fn()} />)

    expect(screen.queryAllByRole('button')).toHaveLength(1)
    expect(screen.getByText(VILLE.labelTh)).toBeInTheDocument()
    expect(screen.queryByText(HOTEL.labelTh)).not.toBeInTheDocument()
  })
})

describe('HkBranchChip', () => {
  it('renders nothing when there is one branch or none — nothing to switch to', () => {
    const { container: single } = render(
      <HkBranchChip branches={[VILLE]} current="hfville" onSwitch={jest.fn()} />
    )
    expect(single).toBeEmptyDOMElement()

    // The enforcement-era case: an empty list must not render a switcher
    // either (and must not crash looking up a label that isn't there).
    const { container: empty } = render(
      <HkBranchChip branches={[]} current="hfhotel" onSwitch={jest.fn()} />
    )
    expect(empty).toBeEmptyDOMElement()
  })
})
