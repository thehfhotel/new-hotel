//! Public installer downloads.
//!
//! Serves the Thai-ID card-reader middleware installers from a bind-mounted,
//! read-only assets directory so reception can grab them from our own origin
//! (`/api/downloads/middleware/...`) instead of a GitHub release — no external
//! login gate, and it stays reachable behind the same Cloudflare Access edge as
//! the rest of the app. Mirrors the `THAI_ID_TEMPLATE_PATH` bind-mount pattern
//! (host `${ASSETS_DIR}/middleware` -> container `/run/assets/middleware`).
//!
//! This router is mounted PUBLIC (merged alongside `/health`, not under
//! `require_auth`): the installer is not sensitive and reception should be able
//! to fetch it before signing in. It touches no database — pure file serving.

use axum::{
    body::Body,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use std::path::{Path as StdPath, PathBuf};

/// Directory holding the middleware installers (bind-mounted read-only).
/// Defaults to the in-container mount point; override with `MIDDLEWARE_ASSETS_DIR`.
fn middleware_dir() -> PathBuf {
    std::env::var("MIDDLEWARE_ASSETS_DIR")
        .unwrap_or_else(|_| "/run/assets/middleware".to_string())
        .into()
}

/// Newest file in `dir` whose extension matches `ext` (case-insensitive),
/// chosen by modified-time so a version bump is served with no code change.
/// `ext` is a fixed literal from the platform whitelist — never user input —
/// and filenames come from the directory listing, so there is no path traversal.
fn newest_with_ext(dir: &StdPath, ext: &str) -> Option<PathBuf> {
    let mut best: Option<(std::time::SystemTime, PathBuf)> = None;
    for entry in std::fs::read_dir(dir).ok()?.flatten() {
        let path = entry.path();
        let is_match = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case(ext))
            .unwrap_or(false);
        if !is_match {
            continue;
        }
        let Ok(mtime) = entry.metadata().and_then(|m| m.modified()) else {
            continue;
        };
        if best.as_ref().map(|(t, _)| mtime > *t).unwrap_or(true) {
            best = Some((mtime, path));
        }
    }
    best.map(|(_, p)| p)
}

pub fn router() -> Router {
    Router::new().route(
        "/api/downloads/middleware/{platform}",
        get(middleware_download),
    )
}

/// `GET /api/downloads/middleware/{platform}` — stream the installer for the
/// requested platform. `platform` is whitelisted to a file extension; anything
/// else is a 404.
async fn middleware_download(Path(platform): Path<String>) -> Response {
    let ext = match platform.as_str() {
        "win" | "windows" | "msi" => "msi",
        "win-exe" | "exe" => "exe", // NSIS installer, kept as an alternative
        "mac" | "macos" | "dmg" => "dmg",
        _ => return (StatusCode::NOT_FOUND, "unknown platform").into_response(),
    };

    let dir = middleware_dir();
    let Some(path) = newest_with_ext(&dir, ext) else {
        return (
            StatusCode::NOT_FOUND,
            "installer not available on this server",
        )
            .into_response();
    };

    let filename = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("thai-id-middleware")
        .to_string();

    match tokio::fs::read(&path).await {
        Ok(bytes) => Response::builder()
            .header(header::CONTENT_TYPE, "application/octet-stream")
            .header(
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{filename}\""),
            )
            .header(header::CACHE_CONTROL, "public, max-age=300")
            .body(Body::from(bytes))
            .unwrap(),
        Err(e) => {
            tracing::error!("middleware download read failed for {}: {e}", path.display());
            (StatusCode::INTERNAL_SERVER_ERROR, "read failed").into_response()
        }
    }
}
