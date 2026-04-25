# syntax=docker/dockerfile:1.7
#
# Frontend image — Next.js standalone build with BuildKit cache mounts on
# pnpm's content-addressed store and Next.js's webpack cache. Both persist
# across CI runs via `cache-to: type=gha,mode=max` on the build-push-action.

FROM node:20-alpine AS base
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
# Next.js writes its incremental webpack/SWC cache to .next/cache. Mounting
# it as a BuildKit cache means re-builds skip work for unchanged modules.
# Output (.next/standalone, .next/static) is NOT under .next/cache, so the
# COPY in the runner stage still finds it.
RUN --mount=type=cache,target=/app/.next/cache,id=next-cache \
    pnpm build

FROM base AS runner
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app/.next/standalone ./
COPY --from=builder /app/.next/static ./.next/static
COPY --from=builder /app/public ./public
COPY --from=builder /app/CHANGELOG.md ./
EXPOSE 3003
ENV PORT=3003
CMD ["node", "server.js"]
