// Spiegelt die JSON-DTOs des Backends (Spec 005/006).

export type Currency = 'EUR' | 'USD';
export type InvoiceStatusLabel = 'Open' | 'Paid' | 'Overpaid';

export interface InvoiceDto {
  id: string;
  invoice_number: string;
  issue_date: string;
  due_date: string;
  amount: string;
  currency: Currency;
  status: InvoiceStatusLabel;
  /** Summe der bisher zugewiesenen Zahlungen (Spec 009, für Teilzahlungen). */
  paid_amount: string;
}

export interface TransactionDto {
  id: string;
  booking_date: string;
  value_date: string;
  amount: string;
  currency: Currency;
  reference_text: string;
  counterparty_name: string | null;
  counterparty_iban: string | null;
}

export interface UploadResult {
  inserted: number;
  skipped: number;
  errors: string[];
}

/** Nachrichten, die über den WebSocket (`/ws`, Spec 007 Backend) hereinkommen. */
export type RealtimeEvent =
  | { type: 'invoice_created'; invoice_number: string; amount: string }
  | { type: 'transaction_created'; reference_text: string; amount: string }
  | { type: 'allocation_created'; invoice_id: string; transaction_id: string; allocated_amount: string };
