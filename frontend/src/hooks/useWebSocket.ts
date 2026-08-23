import { useEffect, useRef } from 'react';
import { getWebSocketUrl } from '../services/api';
import type { RealtimeEvent } from '../services/types';

const MAX_RECONNECT_ATTEMPTS = 5;

/**
 * Verbindet sich mit dem Backend-WebSocket (`/ws`, Spec 007 Backend) und ruft `onEvent` für
 * jede eingehende Nachricht auf. Verbindungsabbrüche lösen einen Reconnect mit exponentiellem
 * Backoff aus (max. 5 Versuche), damit ein kurz nicht erreichbares Backend die UI nicht crasht.
 */
export function useWebSocket(onEvent: (event: RealtimeEvent) => void) {
  // Ref statt State: der Callback soll bei jedem Render aktuell sein, ohne den Socket neu aufzubauen.
  const onEventRef = useRef(onEvent);
  onEventRef.current = onEvent;

  useEffect(() => {
    let socket: WebSocket | null = null;
    let attempt = 0;
    let closedByCleanup = false;
    let reconnectTimer: ReturnType<typeof setTimeout> | undefined;

    const connect = () => {
      try {
        socket = new WebSocket(getWebSocketUrl());
      } catch (error) {
        console.error('WebSocket-Verbindung konnte nicht aufgebaut werden', error);
        return;
      }

      socket.onmessage = (message) => {
        try {
          const parsed = JSON.parse(message.data) as RealtimeEvent;
          onEventRef.current(parsed);
        } catch (error) {
          console.error('Ungültiges WebSocket-Event empfangen', error);
        }
      };

      socket.onclose = () => {
        if (closedByCleanup || attempt >= MAX_RECONNECT_ATTEMPTS) {
          return;
        }
        attempt += 1;
        const backoffMs = Math.min(1000 * 2 ** attempt, 10_000);
        reconnectTimer = setTimeout(connect, backoffMs);
      };

      socket.onerror = () => {
        // Der Fehler führt automatisch zu `onclose`, daher hier nur Logging.
        console.warn('WebSocket-Fehler, versuche erneut zu verbinden…');
      };
    };

    connect();

    return () => {
      closedByCleanup = true;
      if (reconnectTimer) clearTimeout(reconnectTimer);
      socket?.close();
    };
  }, []);
}
