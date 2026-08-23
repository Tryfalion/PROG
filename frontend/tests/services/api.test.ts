import { describe, expect, it, vi, beforeEach } from 'vitest';
import { getInvoices } from '../../src/services/api';

describe('services/api', () => {
  beforeEach(() => {
    vi.restoreAllMocks();
  });

  it('getInvoices parses a mocked fetch response correctly', async () => {
    const mockInvoices = [
      { id: '1', invoice_number: 'RE-1', issue_date: '2026-06-01', due_date: '2026-06-14', amount: '150.00', currency: 'EUR', status: 'Open' },
    ];
    vi.stubGlobal(
      'fetch',
      vi.fn().mockResolvedValue({ ok: true, json: async () => mockInvoices })
    );

    const result = await getInvoices();
    expect(result).toEqual(mockInvoices);
  });

  it('throws when the backend responds with a non-OK status', async () => {
    vi.stubGlobal('fetch', vi.fn().mockResolvedValue({ ok: false, status: 500, json: async () => ({}) }));
    await expect(getInvoices()).rejects.toThrow();
  });
});
