import { describe, expect, it, vi } from 'vitest';
import { render, waitFor } from '@testing-library/react';
import { useWebSocket } from '../../src/hooks/useWebSocket';

/** Minimaler WebSocket-Mock, der sofort ein Event feuert, sobald `send` (hier: Testtrigger) passiert. */
class MockWebSocket {
  onmessage: ((event: { data: string }) => void) | null = null;
  onclose: (() => void) | null = null;
  onerror: (() => void) | null = null;
  static instances: MockWebSocket[] = [];

  constructor(_url: string) {
    MockWebSocket.instances.push(this);
  }

  close() {}
}

function TestComponent({ onEvent }: { onEvent: (e: unknown) => void }) {
  useWebSocket(onEvent as never);
  return null;
}

describe('useWebSocket', () => {
  it('calls onEvent when a message is received', async () => {
    vi.stubGlobal('WebSocket', MockWebSocket as unknown as typeof WebSocket);
    const onEvent = vi.fn();

    render(<TestComponent onEvent={onEvent} />);

    const socket = MockWebSocket.instances[MockWebSocket.instances.length - 1];
    socket.onmessage?.({ data: JSON.stringify({ type: 'invoice_created', invoice_number: 'RE-1', amount: '10.00' }) });

    await waitFor(() => {
      expect(onEvent).toHaveBeenCalledWith({ type: 'invoice_created', invoice_number: 'RE-1', amount: '10.00' });
    });
  });
});
