'use client'

// Branch picker for the maid-facing housekeeping surface (/hk) — wave-4 §A.
//
// Two shapes, both driven by `GET /api/hk/me`'s `branches[]` (the ONE source
// of truth for `HK_BRANCHES`, no NEXT_PUBLIC_* coupling):
//
//   HkBranchPicker — full-screen, blocking. Shown when nothing valid is
//   stored yet (`resolveInitialBranch` returned `null`). Renders NOTHING
//   that could be mistaken for a default — the maid must tap one.
//
//   HkBranchChip — a small header switcher for changing branch at any time
//   (estate precedent `~/HF/housekeeping/src/client/staff/App.tsx:103-108`).
//   Renders nothing at all when only one branch is configured, matching
//   §A3.1 ("branches.length === 1 ⇒ auto-select, no picker rendered — this
//   is the shipping state, maids see zero new UI").

import { useState } from 'react'
import { MapPin } from 'lucide-react'
import type { Branch, HkBranchOption } from './hk-lib'

interface HkBranchPickerProps {
  branches: HkBranchOption[]
  onPick: (branch: Branch) => void
}

export function HkBranchPicker({ branches, onPick }: HkBranchPickerProps) {
  return (
    <section className="mx-auto max-w-md px-2 py-8">
      <h2 className="mb-1 text-center text-lg font-semibold text-gray-900">
        เลือกสาขาที่ทำงานอยู่
      </h2>
      <p className="mb-6 text-center text-sm text-gray-500">
        เลือกครั้งเดียว ระบบจะจำไว้ให้ในเครื่องนี้
      </p>
      <div className="grid gap-3">
        {branches.map((b) => (
          <button
            key={b.id}
            type="button"
            onClick={() => onPick(b.id)}
            className="flex items-center justify-center gap-2 rounded-xl border border-gray-200 bg-white px-6 py-6 text-lg font-medium text-gray-900 shadow-sm active:bg-gray-50"
          >
            <MapPin className="h-5 w-5 text-red-600" />
            {b.labelTh}
          </button>
        ))}
      </div>
    </section>
  )
}

interface HkBranchChipProps {
  branches: HkBranchOption[]
  current: Branch
  onSwitch: (branch: Branch) => void
}

export function HkBranchChip({ branches, current, onSwitch }: HkBranchChipProps) {
  const [open, setOpen] = useState(false)

  // Single-branch ⇒ no picker rendered at all (§A3.1) — nothing to switch to.
  if (branches.length <= 1) return null

  const label = branches.find((b) => b.id === current)?.labelTh ?? current

  return (
    <div className="relative">
      <button
        type="button"
        onClick={() => setOpen((o) => !o)}
        aria-expanded={open}
        className="flex items-center gap-1 rounded-full border border-red-300 bg-red-50 px-3 py-1 text-xs font-medium text-red-700"
      >
        <MapPin className="h-3.5 w-3.5" />
        {label}
      </button>
      {open && (
        <>
          {/* Click-outside catcher */}
          <button
            type="button"
            aria-label="ปิด"
            onClick={() => setOpen(false)}
            className="fixed inset-0 z-10 cursor-default"
          />
          <div className="absolute right-0 z-20 mt-1 w-44 overflow-hidden rounded-lg border border-gray-200 bg-white py-1 shadow-lg">
            {branches.map((b) => (
              <button
                key={b.id}
                type="button"
                onClick={() => {
                  setOpen(false)
                  if (b.id !== current) onSwitch(b.id)
                }}
                className={`block w-full px-3 py-2 text-left text-sm active:bg-gray-50 ${
                  b.id === current ? 'font-semibold text-red-700' : 'text-gray-700'
                }`}
              >
                {b.labelTh}
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  )
}
