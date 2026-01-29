import { NextResponse } from 'next/server';
import { promises as fs } from 'fs';
import path from 'path';

export const dynamic = 'force-dynamic';

interface Release {
  tag: string;
  name: string;
  body: string;
  publishedAt: string;
  url: string;
}

interface ParsedEntry {
  version: string;
  date: string;
  content: string;
}

function parseChangelog(content: string): ParsedEntry[] {
  const entries: ParsedEntry[] = [];
  const lines = content.split('\n');

  let currentEntry: ParsedEntry | null = null;
  let contentLines: string[] = [];

  for (const line of lines) {
    // Match version headers like "## [1.15.4] - 2026-01-29"
    const versionMatch = line.match(/^## \[([^\]]+)\] - (\d{4}-\d{2}-\d{2})/);

    if (versionMatch) {
      // Save previous entry if exists
      if (currentEntry) {
        currentEntry.content = contentLines.join('\n').trim();
        entries.push(currentEntry);
      }

      // Start new entry
      currentEntry = {
        version: versionMatch[1],
        date: versionMatch[2],
        content: '',
      };
      contentLines = [];
    } else if (currentEntry) {
      // Skip the main header and description lines at the top
      contentLines.push(line);
    }
  }

  // Don't forget the last entry
  if (currentEntry) {
    currentEntry.content = contentLines.join('\n').trim();
    entries.push(currentEntry);
  }

  return entries;
}

export async function GET() {
  try {
    // Read the local CHANGELOG.md file
    const changelogPath = path.join(process.cwd(), 'CHANGELOG.md');
    const content = await fs.readFile(changelogPath, 'utf-8');

    const entries = parseChangelog(content);

    const releases: Release[] = entries.map((entry) => ({
      tag: `v${entry.version}`,
      name: `v${entry.version}`,
      body: entry.content,
      publishedAt: new Date(entry.date).toISOString(),
      url: `https://github.com/thehfhotel/new-hotel/releases/tag/v${entry.version}`,
    }));

    return NextResponse.json({
      success: true,
      releases,
    });
  } catch (error) {
    console.error('Error reading changelog:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to read changelog',
        releases: [],
      },
      { status: 500 }
    );
  }
}
