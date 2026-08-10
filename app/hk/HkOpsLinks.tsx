'use client'

// Deep links into the separate Housekeeping ops app (repo `~/housekeeping`,
// housekeeping.thehfhotel.org), which now owns breakage reports (แจ้งซ่อม)
// and stock requisitions (เบิกของ) — retired from this /hk surface. Rendered
// on both the room list and room detail screens so either action is reachable
// without a room context.

import { Package, Wrench } from 'lucide-react'
import { HOUSEKEEPING_URL } from './hk-lib'

export default function HkOpsLinks() {
  return (
    <section className="mb-6">
      <h2 className="mb-2 flex items-center gap-1.5 text-sm font-semibold text-gray-600">
        งานอื่น ๆ
      </h2>
      <div className="grid grid-cols-2 gap-3">
        <a
          href={`${HOUSEKEEPING_URL}/staff/report`}
          className="block rounded-xl border border-orange-300 bg-orange-50 px-3 py-4 text-center text-base font-semibold text-orange-800 active:bg-orange-100"
        >
          <span className="flex items-center justify-center gap-2">
            <Wrench className="h-5 w-5" />
            แจ้งซ่อม
          </span>
        </a>
        <a
          href={`${HOUSEKEEPING_URL}/staff/stock`}
          className="block rounded-xl border border-sky-300 bg-sky-50 px-3 py-4 text-center text-base font-semibold text-sky-800 active:bg-sky-100"
        >
          <span className="flex items-center justify-center gap-2">
            <Package className="h-5 w-5" />
            เบิกของ
          </span>
        </a>
      </div>
    </section>
  )
}
