import type { InvoiceDto, TransactionDto, UploadResult } from './types';

// Basis-URL des Backends. Über eine Vite-Umgebungsvariable konfigurierbar, damit Dev/Prod/Docker
// unterschiedliche Backend-Adressen nutzen können (siehe Spec 006).
const API_BASE_URL = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:3000';

/** Leitet die WebSocket-URL aus der API-Basis-URL ab (http -> ws, https -> wss). */
export function getWebSocketUrl(): string {
  return API_BASE_URL.replace(/^http/, 'ws') + '/ws';
}

async function requestJson<T>(path: string, init?: RequestInit): Promise<T> {
  const response = await fetch(`${API_BASE_URL}${path}`, init);
  if (!response.ok) {
    throw new Error(`Anfrage an ${path} fehlgeschlagen: ${response.status}`);
  }
  return (await response.json()) as T;
}

export function getInvoices(): Promise<InvoiceDto[]> {
  return requestJson<InvoiceDto[]>('/api/v1/invoices');
}

export function getTransactions(): Promise<TransactionDto[]> {
  return requestJson<TransactionDto[]>('/api/v1/transactions');
}

export function createInvoice(payload: { invoice_number: string; amount: string; currency: string }): Promise<Response> {
  return fetch(`${API_BASE_URL}/api/v1/invoices`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
}

export function createTransaction(payload: {
  booking_date: string;
  value_date: string;
  amount: string;
  currency: string;
  reference_text: string;
  counterparty_name?: string | null;
  counterparty_iban?: string | null;
}): Promise<Response> {
  return fetch(`${API_BASE_URL}/api/v1/transactions`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(payload),
  });
}

/** Lädt eine Datei per Drag & Drop / Dateiauswahl zum passenden Upload-Endpunkt hoch (Spec 007). */
export async function uploadFile(kind: 'invoice' | 'payment', file: File): Promise<UploadResult> {
  const endpoint = kind === 'invoice' ? '/api/v1/invoices/upload' : '/api/v1/transactions/upload';
  const formData = new FormData();
  formData.append('file', file);

  return requestJson<UploadResult>(endpoint, { method: 'POST', body: formData });
}
