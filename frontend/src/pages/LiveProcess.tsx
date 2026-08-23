import React, { useMemo, useState } from 'react';
import { useInvoicesAndTransactions } from '../hooks/useInvoicesAndTransactions';
import { FileDropModal } from '../components/FileDropModal';
import { KpiCard } from '../components/KpiCard';
import type { InvoiceDto } from '../services/types';

/** Übersetzt den N2-Status einer Rechnung in das Vokabular des Reconciliation Boards (Spec 009). */
const reconciliationLabel = (status: InvoiceDto['status']): { text: string; className: string } => {
  switch (status) {
    case 'Paid':
      return { text: 'Perfect Match', className: 'status-paid' };
    case 'Overpaid':
      return { text: 'Overpayment', className: 'status-warn' };
    case 'Open':
    default:
      return { text: 'Open / Unpaid', className: 'status-open' };
  }
};

export const LiveProcess: React.FC = () => {
  const { invoices, transactions, error } = useInvoicesAndTransactions();
  const [invoiceSearch, setInvoiceSearch] = useState('');
  const [modalKind, setModalKind] = useState<'invoice' | 'payment' | null>(null);

  const filteredInvoices = useMemo(
    () => invoices.filter((inv) => inv.invoice_number.toLowerCase().includes(invoiceSearch.toLowerCase())),
    [invoices, invoiceSearch]
  );

  // KPI-Berechnungen gemäß Spec 009.
  const totalInvoiced = invoices.reduce((sum, inv) => sum + Number(inv.amount), 0);
  const totalDeposits = transactions.reduce((sum, tx) => sum + Number(tx.amount), 0);
  const paidCount = invoices.filter((inv) => inv.status === 'Paid' || inv.status === 'Overpaid').length;
  const matchedRate = invoices.length === 0 ? 0 : Math.round((paidCount / invoices.length) * 100);
  const openDiscrepancies = invoices.filter((inv) => inv.status !== 'Paid').length;

  return (
    <div>
      <h1 style={{ color: 'var(--color-secondary-blue)' }}>Invoice &amp; Payments Comparison</h1>
      <p>Audit accounts receivable, automatically pair cash logs, and trigger reconciliation workflows.</p>

      {error && <p className="upload-error">Verbindung zum Backend fehlgeschlagen: {error}</p>}

      <div className="kpi-row">
        <KpiCard label="Total Invoiced" value={`${totalInvoiced.toFixed(2)} €`} sublabel="Aus aktivem Rechnungs-Ledger" />
        <KpiCard label="Deposits Received" value={`${totalDeposits.toFixed(2)} €`} sublabel="Alle importierten Zahlungen" />
        <KpiCard label="Matched Collection Rate" value={`${matchedRate}%`} sublabel="Bezahlt vs. Rechnungen gesamt" />
        <KpiCard
          label="Open Discrepancies"
          value={String(openDiscrepancies)}
          sublabel="Unbezahlt, überzahlt oder ausstehend"
          accent={openDiscrepancies > 0 ? 'warn' : 'default'}
        />
      </div>

      <div className="board-grid">
        {/* Spalte 1: Invoices Ledger */}
        <div className="card">
          <div className="ledger-header">
            <h2>Invoices Ledger</h2>
            <button className="btn-primary" onClick={() => setModalKind('invoice')}>
              + Add Invoice
            </button>
          </div>
          <input
            className="search-input"
            placeholder="Search invoice number…"
            value={invoiceSearch}
            onChange={(e) => setInvoiceSearch(e.target.value)}
          />
          <ul className="ledger-list">
            {filteredInvoices.map((inv) => (
              <li key={inv.id} className="ledger-item">
                <div>
                  <b>{inv.invoice_number}</b>
                  <div className="ledger-sub">Fällig: {inv.due_date}</div>
                </div>
                <div>{inv.amount} {inv.currency}</div>
              </li>
            ))}
            {filteredInvoices.length === 0 && <li className="ledger-empty">Keine Rechnungen gefunden.</li>}
          </ul>
        </div>

        {/* Spalte 2: Payments Ledger */}
        <div className="card">
          <div className="ledger-header">
            <h2>Payments Ledger</h2>
            <button className="btn-primary" onClick={() => setModalKind('payment')}>
              + Add Payment
            </button>
          </div>
          <ul className="ledger-list">
            {transactions.map((tx) => (
              <li key={tx.id} className="ledger-item">
                <div>
                  <b>{tx.reference_text}</b>
                  <div className="ledger-sub">{tx.counterparty_name ?? 'Unbekannter Absender'}</div>
                </div>
                <div>{tx.amount} {tx.currency}</div>
              </li>
            ))}
            {transactions.length === 0 && <li className="ledger-empty">Keine Zahlungen gefunden.</li>}
          </ul>
        </div>

        {/* Spalte 3: Reconciliation Board */}
        <div className="card" style={{ backgroundColor: '#F0F4F8' }}>
          <h2>Reconciliation Board</h2>
          <ul className="ledger-list">
            {invoices.map((inv) => {
              const label = reconciliationLabel(inv.status);
              return (
                <li key={inv.id} className="reconciliation-card">
                  <span className={`status-badge ${label.className}`}>{label.text}</span>
                  <div className="ledger-sub">{inv.invoice_number} — {inv.amount} {inv.currency}</div>
                </li>
              );
            })}
            {invoices.length === 0 && <li className="ledger-empty">Noch keine Rechnungen zum Abgleich.</li>}
          </ul>
        </div>
      </div>

      {modalKind && (
        <FileDropModal
          kind={modalKind}
          onClose={() => setModalKind(null)}
          onUploaded={() => {
            /* Listen aktualisieren sich per WebSocket automatisch (Spec 006/007). */
          }}
        />
      )}
    </div>
  );
};
