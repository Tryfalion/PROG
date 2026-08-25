import React from 'react';
import { useInvoicesAndTransactions } from '../hooks/useInvoicesAndTransactions';

// Zeigt reale Kennzahlen aus dem Backend an (Spec 009), statt der ursprünglichen Mock-Werte.
export const Statistics: React.FC = () => {
  const { invoices, transactions, error } = useInvoicesAndTransactions();

  const paidCount = invoices.filter((inv) => inv.status === 'Paid').length;
  const overpaidCount = invoices.filter((inv) => inv.status === 'Overpaid').length;
  const openCount = invoices.filter((inv) => inv.status === 'Open').length;
  // "Eingenommen" = tatsächlich zugewiesene Beträge über alle Rechnungen hinweg,
  // damit auch Teilzahlungen offener Rechnungen mitgezählt werden (nicht nur volle Zahlungen).
  const amountPaid = invoices.reduce((sum, inv) => sum + (Number(inv.paid_amount) || 0), 0);
  // Teilzahlungen mindern den offenen Betrag: nur (Rechnungsbetrag - bereits Zugewiesenes) zählt als ausstehend.
  // Number(undefined) fällt auf 0 zurück, falls ein älteres Backend ohne paid_amount läuft.
  const amountOpen = invoices
    .filter((inv) => inv.status === 'Open')
    .reduce((sum, inv) => sum + (Number(inv.amount) - (Number(inv.paid_amount) || 0)), 0);

  return (
    <div>
      <h1 style={{ color: 'var(--color-secondary-blue)' }}>Finanz-Statistiken</h1>
      {error && <p className="upload-error">Verbindung zum Backend fehlgeschlagen: {error}</p>}

      <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
        <div className="card">
          <h3>Rechnungsvolumen</h3>
          <p>Gesamt: <b>{invoices.length}</b></p>
          <p>Bezahlt: <span style={{ color: 'var(--status-paid)' }}>{paidCount}</span></p>
          <p>Überzahlt: <span style={{ color: 'var(--status-warn)' }}>{overpaidCount}</span></p>
          <p>Offen: <span style={{ color: 'var(--status-open)' }}>{openCount}</span></p>
        </div>

        <div className="card">
          <h3>Zahlungsvolumen</h3>
          <p>Eingänge: <b>{transactions.length}</b></p>
        </div>

        <div className="card">
          <h3>Monetäre Übersicht</h3>
          <p>Eingenommen: <b>{amountPaid.toFixed(2)} EUR</b></p>
          <p>Ausstehend: <span style={{ color: 'var(--status-open)' }}>{amountOpen.toFixed(2)} EUR</span></p>
        </div>
      </div>
    </div>
  );
};
