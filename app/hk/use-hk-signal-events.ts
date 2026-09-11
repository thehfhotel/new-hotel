'use client'

// Live room signals on the /hk surface (ADR 0008 §Decision 2: "routine
// delivery rides our own rails and consumes zero LINE messages").
//
// WHY THIS EXISTS AT ALL. `use-hk-auto-refresh.ts` says, in its own header,
// that /hk cannot use SSE because the reception board's `/api/events` sits
// behind the cookie-session middleware, OUTSIDE the path-scoped `/hk`
// Cloudflare Access application. That is still true of THAT stream — which is
// exactly why the signals build adds a /hk-native one: `/hk/api/events`, which
// next.config.js rewrites to the backend's `/api/hk/events` and which
// therefore carries the /hk app's own Access assertion, like every other call
// this surface makes. The sixty-second poll stays as the fallback beneath it.
//
// WHY THE CUE. A maid is holding the phone in one hand and linen in the other,
// with the screen dark most of the time; a ขอเช็คห้อง means a guest is standing
// at the counter waiting for her answer. A silent row appearing in a list she
// is not looking at is not delivery. The cue is a single short tone, no asset
// file (WebAudio only — nothing to ship, nothing to cache, nothing to 404),
// and it fires ONLY for a brand-new signal pointed at this role: never for the
// user's own taps, never for the ack/done echoes of a signal already on
// screen. A cue that fires for your own thumb is a cue that gets muted.
//
// FAILURE POSTURE. Everything here is best-effort and silent. No EventSource
// (jsdom, an old WebView), a blocked AudioContext (iOS autoplay policy before
// the first gesture), a stream that never connects behind a captive portal —
// each degrades to "no live push, no sound", and `live === false` is what
// tells the page to keep polling. An SSE failure must never be able to take a
// maid's screen down; that is why nothing in here ever throws or renders.

import { useEffect, useRef, useState } from 'react'

import {
  HK_API_BASE,
  readSignalSoundMuted,
  type Branch,
  type RoomSignal,
} from './hk-lib'

/** The one event name the backend emits on this stream (`hk_signal`, data = a
 * `RoomSignal` DTO). Keepalives arrive as comments and never reach a
 * listener. */
export const HK_SIGNAL_EVENT = 'hk_signal'

/** Cue shape: a short, quiet two-note blip. Long enough to be heard over a
 * corridor, short enough that nobody learns to resent it. */
const CUE_FREQUENCIES_HZ = [880, 1320]
const CUE_NOTE_SECONDS = 0.12
const CUE_GAIN = 0.12

type AudioContextCtor = typeof AudioContext

let audioContext: AudioContext | null = null

function resolveAudioContext(): AudioContext | null {
  if (typeof window === 'undefined') return null
  try {
    const Ctor: AudioContextCtor | undefined =
      window.AudioContext ??
      (window as unknown as { webkitAudioContext?: AudioContextCtor }).webkitAudioContext
    if (!Ctor) return null
    if (!audioContext) audioContext = new Ctor()
    return audioContext
  } catch {
    return null
  }
}

/**
 * Play the new-signal cue, unless the page-level mute is on. Best-effort and
 * never throws: a browser that refuses to start an AudioContext before the
 * first user gesture simply produces no sound, and the row still appears.
 *
 * The mute is read from storage at PLAY time rather than passed in, so both
 * /hk screens honour one toggle without either having to own the state.
 */
export function playHkSignalCue(): void {
  if (readSignalSoundMuted()) return
  const ctx = resolveAudioContext()
  if (!ctx) return
  try {
    // Suspended is the normal state before the first gesture on iOS; resuming
    // is best-effort and its rejection is not an error worth surfacing.
    void ctx.resume?.()?.catch?.(() => {})
    CUE_FREQUENCIES_HZ.forEach((frequency, index) => {
      const startAt = ctx.currentTime + index * CUE_NOTE_SECONDS
      const endAt = startAt + CUE_NOTE_SECONDS
      const oscillator = ctx.createOscillator()
      const gain = ctx.createGain()
      oscillator.type = 'sine'
      oscillator.frequency.value = frequency
      // Ramp down rather than cutting: an abrupt stop clicks on phone speakers.
      gain.gain.setValueAtTime(CUE_GAIN, startAt)
      gain.gain.exponentialRampToValueAtTime(0.0001, endAt)
      oscillator.connect(gain)
      gain.connect(ctx.destination)
      oscillator.start(startAt)
      oscillator.stop(endAt)
    })
  } catch {
    /* no sound; the signal still lands on screen */
  }
}

/**
 * Subscribe to this branch's signal stream and hand every `hk_signal` DTO to
 * `onSignal`. Returns whether the stream is currently CONNECTED — the page
 * uses it to decide whether the sixty-second poll still has to run.
 *
 * `onSignal` is held in a ref, so a caller whose closure changes every render
 * does not tear down and re-open the stream (the same pattern
 * `useHkAutoRefresh` and `useLiveRefresh` both use). The subscription resets
 * only when `branch` or `enabled` changes.
 *
 * A malformed payload is DROPPED, not thrown: one bad frame from a
 * mid-deploy backend must not take out the listener for every later one.
 */
export function useHkSignalEvents(
  branch: Branch | null,
  onSignal: (signal: RoomSignal) => void,
  enabled: boolean = true
): boolean {
  const [live, setLive] = useState(false)
  const onSignalRef = useRef(onSignal)

  useEffect(() => {
    onSignalRef.current = onSignal
  })

  useEffect(() => {
    if (!branch || !enabled) return
    // jsdom, and any WebView old enough to lack EventSource: the poll is
    // already the fallback, so this is a no-op rather than a failure.
    if (typeof EventSource === 'undefined') return

    let source: EventSource
    try {
      source = new EventSource(
        `${HK_API_BASE}/events?branch=${encodeURIComponent(branch)}`
      )
    } catch {
      return
    }

    const handle = (event: MessageEvent) => {
      try {
        const signal: RoomSignal = JSON.parse(event.data)
        if (signal && typeof signal.signalId === 'number') onSignalRef.current(signal)
      } catch {
        /* one unreadable frame changes nothing about the next one */
      }
    }

    source.addEventListener(HK_SIGNAL_EVENT, handle as EventListener)
    source.onopen = () => setLive(true)
    // EventSource reconnects on its own; `live: false` in the meantime is what
    // hands the job back to the poll, so a dropped stream costs at most one
    // poll interval of latency instead of the page's liveness.
    source.onerror = () => setLive(false)

    return () => {
      source.removeEventListener(HK_SIGNAL_EVENT, handle as EventListener)
      try {
        source.close()
      } catch {
        /* already closed */
      }
      setLive(false)
    }
  }, [branch, enabled])

  return live
}
