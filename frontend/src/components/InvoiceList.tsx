import React from 'react';

// Mirroring the Backend Schema (API Schema)
export type Currency = 'EUR' | 'USD';
export type InvoiceStatus = 'Open' | 'Paid' | 'Overpaid';

export interface Invoice {
  id: string;
  invoice_number: string;
  issue_date: string;
  due_date: string;
  amount: string; // Decimal parsed as String via API
  currency: Currency;
  status: InvoiceStatus;
}

interface Props {
  invoices: Invoice[];
}

export const InvoiceList: React.FC<Props> = ({ invoices }) => {
  // Eine sehr einfache, leicht verständliche Formatter-Funktion
  const renderStatus = (status: InvoiceStatus) => {
    switch (status) {
      case 'Paid':
        return <span style={{ color: 'green', fontWeight: 'bold' }}>Bezahlt</span>;
      case 'Overpaid':
        return <span style={{ color: 'red', fontWeight: 'bold' }}>Überzahlt (Aktion nötig)</span>;
      case 'Open':
      default:
        return <span style={{ color: 'orange', fontWeight: 'bold' }}>Offen</span>;
    }
  };

  return (
    <div>
      <h2>Rechnungs-Übersicht</h2>
      {invoices.length === 0 ? (
        <p>Aktuell keine Rechnungen vorhanden.</p>
      ) : (
        <table style={{ width: '100%', textAlign: 'left', borderCollapse: 'collapse' }}>
          <thead>
            <tr style={{ borderBottom: '2px solid #ccc' }}>
              <th>Rechnungsnummer</th>
              <th>Status</th>
              <th>Bruttobetrag</th>
            </tr>
          </thead>
          <tbody>
            {invoices.map((inv) => (
              <tr key={inv.id} style={{ borderBottom: '1px solid #eee' }}>
                <td>{inv.invoice_number}</td>
                <td>{renderStatus(inv.status)}</td>
                <td>{inv.amount} {inv.currency}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </div>
  );
};
