# syntax=docker/dockerfile:1.7
#
# Frontend image — Next.js standalone build with BuildKit cache mounts on
# pnpm's content-addressed store and Next.js's webpack cache. Both persist
# across CI runs via `cache-to: type=gha,mode=max` on the build-push-action.

# Pin to digest (Batch C of post-Phase-5.5 audit). Source: Docker Hub API
# `hub.docker.com/v2/repositories/library/node/tags/20-alpine` on 2026-04-26.
# Dependabot (`docker` ecosystem, see .github/dependabot.yml) will bump this.
FROM node:20-alpine@sha256:fb4cd12c85ee03686f6af5362a0b0d56d50c58a04632e6c0fb8363f609372293 AS base
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary
RUN corepack enable && corepack prepare pnpm@10 --activate
# pnpm puts its store under $HOME/.local/share/pnpm by default; setting
# PNPM_STORE_DIR explicitly ensures the cache mount path matches even if
# HOME ever changes (e.g. switching to a non-root user).
ENV PNPM_STORE_DIR=/root/.local/share/pnpm/store

FROM base AS deps
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
# Mount the pnpm store as a cache so package downloads + content-addressed
# layout persist across builds. node_modules itself stays in the layer.
RUN --mount=type=cache,target=/root/.local/share/pnpm/store,id=pnpm-store \
    pnpm install --frozen-lockfile

FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
ARG NEXT_PUBLIC_CARD_READER_URL
ENV NEXT_PUBLIC_CARD_READER_URL=$NEXT_PUBLIC_CARD_READER_URL
# AuthGuard runtime gate (Phase 4 cutover). When `'true'` the frontend
# enforces the /login redirect on any unauthenticated page navigation.
# When unset/anything-else: AuthGuard is a no-op (auth-off escape hatch
# for local dev). NEXT_PUBLIC_* values are inlined at build time, so a
# flip from false→true requires this image to rebuild.
ARG NEXT_PUBLIC_AUTH_REQUIRED
ENV NEXT_PUBLIC_AUTH_REQUIRED=$NEXT_PUBLIC_AUTH_REQUIRED
# Next.js writes its incremental webpack/SWC cache to .next/cache. Mounting
# it as a BuildKit cache means re-builds skip work for unchanged modules.
# Output (.next/standalone, .next/static) is NOT under .next/cache, so the
# COPY in the runner stage still finds it.
RUN --mount=type=cache,target=/app/.next/cache,id=next-cache \
    pnpm build

# Runner stage — bare node:20-alpine (NOT FROM base): no pnpm in the
# runtime image, just the standalone server output. Smaller surface +
# nothing for an attacker to leverage if they get RCE on the web pod.
FROM node:20-alpine@sha256:fb4cd12c85ee03686f6af5362a0b0d56d50c58a04632e6c0fb8363f609372293 AS runner
LABEL org.opencontainers.image.source=https://github.com/thehfhotel/new-hotel
LABEL org.opencontainers.image.licenses=Proprietary
WORKDIR /app
ENV NODE_ENV=production
# Copy output, then chown to the non-root user we're about to switch to.
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
COPY --from=builder /app/public ./public
COPY --from=builder /app/CHANGELOG.md ./

# Drop root for runtime (mirrors the backend pattern). UID 1001 picked to
# avoid collision with the backend's UID 1000 if both ever land on the
# same host.
RUN addgroup -g 1001 -S nextjs \
    && adduser -S -D -H -u 1001 -G nextjs nextjs \
    && chown -R nextjs:nextjs /app
USER nextjs

EXPOSE 3003
ENV PORT=3003
# Container-level healthcheck (compose-level health is in docker-compose.yml).
# Belt+braces: even outside compose (e.g. local `docker run`), `docker ps`
# surfaces the health state. node:alpine ships wget by default.
HEALTHCHECK --interval=30s --timeout=10s --start-period=30s --retries=3 \
    CMD wget --quiet --tries=1 --spider http://localhost:3003 || exit 1
CMD ["node", "server.js"]
