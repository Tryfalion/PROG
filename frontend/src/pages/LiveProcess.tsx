import React from 'react';

export const LiveProcess: React.FC = () => {
  // Mocks für die Demonstration
  const invoices = [
    { id: '1', num: 'RE-2026-001', amount: '150.00', status: 'Paid' },
    { id: '2', num: 'RE-2026-002', amount: '300.50', status: 'Open' },
    { id: '3', num: 'RE-2026-003', amount: '200.00', status: 'Overpaid' }
  ];

  const getStatusClass = (status: string) => {
    if (status === 'Paid') return 'status-badge status-paid';
    if (status === 'Open') return 'status-badge status-open';
    return 'status-badge status-warn'; // Partial oder Overpaid
  };

  return (
    <div>
      <h1 style={{ color: 'var(--color-secondary-blue)' }}>Live Inbox (Rechnungen & Zahlungen)</h1>
      <p>Prozesse laufen automatisch. Aktualisierungen erscheinen live.</p>

      <div style={{ display: 'flex', gap: '20px' }}>
        {/* Rechnungen */}
        <div className="card" style={{ flex: 2 }}>
          <h2>Rechnungs-Eingang</h2>
          <table style={{ width: '100%', textAlign: 'left' }}>
            <thead>
              <tr style={{ borderBottom: '2px solid #ccc' }}>
                <th>Rechnungsnummer</th>
                <th>Betrag</th>
                <th>Status (N2)</th>
              </tr>
            </thead>
            <tbody>
              {invoices.map(i => (
                <tr key={i.id} style={{ height: '40px', borderBottom: '1px solid #eee' }}>
                  <td>{i.num}</td>
                  <td>{i.amount} €</td>
                  <td><span className={getStatusClass(i.status)}>{i.status === 'Open' ? 'Nicht bezahlt' : i.status === 'Paid' ? 'Bezahlt' : 'Teil/Überbezahlt'}</span></td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>

        {/* Bank Stream */}
        <div className="card" style={{ flex: 1, backgroundColor: '#f0f4f8' }}>
          <h2>Bank Stream (Live)</h2>
          <ul style={{ listStyle: 'none', padding: 0 }}>
            <li style={{ padding: '10px', background: 'white', marginBottom: '5px', borderRadius: '4px' }}>+ 150.00 € (Zahlung RE-2026-001)</li>
            <li style={{ padding: '10px', background: 'white', marginBottom: '5px', borderRadius: '4px' }}>+ 250.00 € (RE-2026-003)</li>
          </ul>
        </div>
      </div>
    </div>
  );
};
