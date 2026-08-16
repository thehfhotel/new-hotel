/**
 * Pin: the PMS's own แจ้งซ่อม (maintenance) UI stays retired.
 *
 * WHY THIS EXISTS
 * The PMS used to ship its own maintenance Kanban at /maintenance, reachable
 * from THREE separate nav surfaces (the v2 "More" hub, the classic Sidebar and
 * the v2 CommandPalette). That board duplicated the separate Housekeeping ops
 * app, which is now the system of record for maintenance work orders. The board
 * and all three nav entries were deleted and collapsed into ONE honest link that
 * leaves the PMS for the Housekeeping app.
 *
 * The failure mode this guards against is quiet: someone re-adds a แจ้งซ่อม
 * entry to a nav list (it looks like a harmless menu item), and reception is
 * silently routed back into the retired duplicate flow — filing work orders
 * into a board nobody reads. So we pin the whole surface, not just the deleted
 * files: exactly ONE nav entry in the entire PMS may say แจ้งซ่อม, and it must
 * be the external Housekeeping-app link.
 *
 * Comments are stripped before counting — /hk files legitimately DESCRIBE the
 * external แจ้งซ่อม app in prose, and comments do not render.
 */

import { existsSync, readFileSync, readdirSync, statSync } from 'fs'
import { join, relative } from 'path'

/** The Thai label for a maintenance request — what reception actually sees. */
const MAINTENANCE_LABEL = 'แจ้งซ่อม'

/** The one file allowed to render it, relative to the repo root. */
const ALLOWED_FILE = 'app/v2/more/page.tsx'

/** Retired route. A nav href pointing here means the retired board came back. */
const RETIRED_HREF_LITERALS = ["'/maintenance'", '"/maintenance"', '`/maintenance`']

const SCANNED_ROOTS = ['app', 'components']

function collectSourceFiles(dir: string, found: string[] = []): string[] {
  for (const entry of readdirSync(dir)) {
    if (entry === 'node_modules' || entry === '.next') continue
    const fullPath = join(dir, entry)
    if (statSync(fullPath).isDirectory()) collectSourceFiles(fullPath, found)
    else if (/\.tsx?$/.test(entry)) found.push(fullPath)
  }
  return found
}

/**
 * Drop comments so only code that can actually RENDER is counted:
 * `/* ... *\/` blocks (this also covers JSX `{/* ... *\/}`), then any line whose
 * trimmed form starts with `//` or `*` (JSDoc continuation lines).
 */
function stripComments(source: string): string {
  return source
    .replace(/\/\*[\s\S]*?\*\//g, '')
    .split('\n')
    .filter((line) => {
      const trimmed = line.trim()
      return !trimmed.startsWith('//') && !trimmed.startsWith('*')
    })
    .join('\n')
}

function countOccurrences(haystack: string, needle: string): number {
  return haystack.split(needle).length - 1
}

const sourceFiles = SCANNED_ROOTS.flatMap((root) =>
  collectSourceFiles(join(process.cwd(), root)),
)

const renderableSourceByPath = new Map<string, string>(
  sourceFiles.map((fullPath) => [
    relative(process.cwd(), fullPath),
    stripComments(readFileSync(fullPath, 'utf8')),
  ]),
)

describe('retired PMS maintenance UI stays retired', () => {
  it('is rendered by exactly one file — the More hub', () => {
    const filesRenderingLabel = [...renderableSourceByPath.entries()]
      .filter(([, source]) => source.includes(MAINTENANCE_LABEL))
      .map(([path]) => path)

    expect(filesRenderingLabel).toEqual([ALLOWED_FILE])
  })

  it('is exactly one nav entry in that file', () => {
    const source = renderableSourceByPath.get(ALLOWED_FILE)!

    const linesMentioningLabel = source
      .split('\n')
      .filter((line) => line.includes(MAINTENANCE_LABEL))

    // ONE line === ONE hub entry. This is the assertion that matters: a second
    // แจ้งซ่อม entry anywhere would land on its own line and trip this.
    expect(linesMentioningLabel).toHaveLength(1)

    // …and exactly ONCE on that line: the label `แจ้งซ่อม (แอปแม่บ้าน)`. The
    // desc deliberately says งานซ่อม rather than แจ้งซ่อม so this count stays 1
    // and the pin reads as what the design asked for — no PMS surface renders
    // แจ้งซ่อม except the single housekeeping-app link.
    expect(countOccurrences(source, MAINTENANCE_LABEL)).toBe(1)
  })

  it('points that entry at the external Housekeeping app, not a PMS route', () => {
    const source = renderableSourceByPath.get(ALLOWED_FILE)!
    const [navEntry] = source.split('\n').filter((l) => l.includes(MAINTENANCE_LABEL))

    expect(navEntry).toContain('HOUSEKEEPING_URL')
    expect(navEntry).toContain('external: true')
  })

  it('has no nav href pointing at the retired /maintenance route', () => {
    const offenders = [...renderableSourceByPath.entries()]
      .filter(([, source]) =>
        RETIRED_HREF_LITERALS.some((literal) => source.includes(literal)),
      )
      .map(([path]) => path)

    expect(offenders).toEqual([])
  })

  it('has no /maintenance page on disk', () => {
    expect(existsSync(join(process.cwd(), 'app/maintenance'))).toBe(false)
  })
})
