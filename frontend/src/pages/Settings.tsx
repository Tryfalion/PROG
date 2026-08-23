import React from 'react';

// Sehr einfache Einstellungsseite (aus dem ursprünglichen Explore als fehlend markiert):
// zeigt die aktuell konfigurierte Backend-Adresse an, damit Nutzer die Verbindung nachvollziehen können.
export const Settings: React.FC = () => {
  const apiBaseUrl = import.meta.env.VITE_API_BASE_URL ?? 'http://localhost:3000';

  return (
    <div>
      <h1 style={{ color: 'var(--color-secondary-blue)' }}>Einstellungen</h1>
      <div className="card" style={{ maxWidth: 480 }}>
        <h3>Backend-Verbindung</h3>
        <p>
          API-Basis-URL: <b>{apiBaseUrl}</b>
        </p>
        <p className="ledger-sub">
          Überschreibbar über die Umgebungsvariable <code>VITE_API_BASE_URL</code> beim Build/Start
          des Frontends (siehe Administrator-Handbuch).
        </p>
      </div>
    </div>
  );
};
