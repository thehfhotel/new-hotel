import { NextResponse } from 'next/server';

export const dynamic = 'force-dynamic';
export const revalidate = 300; // Cache for 5 minutes

interface GitHubRelease {
  tag_name: string;
  name: string;
  body: string;
  published_at: string;
  html_url: string;
}

interface Release {
  tag: string;
  name: string;
  body: string;
  publishedAt: string;
  url: string;
}

export async function GET() {
  try {
    const response = await fetch(
      'https://api.github.com/repos/thehfhotel/new-hotel/releases',
      {
        headers: {
          'Accept': 'application/vnd.github.v3+json',
          'User-Agent': 'new-hotel-app',
        },
        next: { revalidate: 300 }, // Cache for 5 minutes
      }
    );

    if (!response.ok) {
      throw new Error(`GitHub API error: ${response.status}`);
    }

    const data: GitHubRelease[] = await response.json();

    const releases: Release[] = data.map((release) => ({
      tag: release.tag_name,
      name: release.name || release.tag_name,
      body: release.body || '',
      publishedAt: release.published_at,
      url: release.html_url,
    }));

    return NextResponse.json({
      success: true,
      releases,
    });
  } catch (error) {
    console.error('Error fetching releases:', error);
    return NextResponse.json(
      {
        success: false,
        error: 'Failed to fetch releases',
        releases: [],
      },
      { status: 500 }
    );
  }
}
