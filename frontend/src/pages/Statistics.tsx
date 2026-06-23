import React from 'react';

// Basis Component für Grid Layout der Spezifikation
export const Statistics: React.FC = () => {
  // Statische Demodaten wie gefordert für das Layout 
  const stats = {
    totalInvoices: 120,
    totalTransactions: 305,
    paidCount: 105,
    openCount: 15,
    amountPaid: '45,210.00 EUR',
    amountOpen: '3,450.50 EUR'
  };

  return (
    <div>
      <h1 style={{ color: 'var(--color-secondary-blue)' }}>Finanz-Statistiken</h1>
      
      <div style={{ display: 'flex', gap: '20px', flexWrap: 'wrap' }}>
        <div className="card">
          <h3>Rechnungsvolumen</h3>
          <p>Gesamt: <b>{stats.totalInvoices}</b></p>
          <p>Bezahlt: <span style={{ color: 'var(--status-paid)' }}>{stats.paidCount}</span></p>
          <p>Offen: <span style={{ color: 'var(--status-open)' }}>{stats.openCount}</span></p>
        </div>

        <div className="card">
          <h3>Zahlungsvolumen</h3>
          <p>Eingänge: <b>{stats.totalTransactions}</b></p>
        </div>

        <div className="card">
          <h3>Monetäre Übersicht</h3>
          <p>Eingenommen: <b>{stats.amountPaid}</b></p>
          <p>Austehend: <span style={{ color: 'var(--status-open)' }}>{stats.amountOpen}</span></p>
        </div>
      </div>
    </div>
  );
};
