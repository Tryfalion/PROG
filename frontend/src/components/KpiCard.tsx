import React from 'react';

interface Props {
  label: string;
  value: string;
  sublabel?: string;
  accent?: 'default' | 'warn';
}

/** Kleine KPI-Kachel für die Kennzahlen-Zeile des Dashboards (Spec 009). */
export const KpiCard: React.FC<Props> = ({ label, value, sublabel, accent = 'default' }) => (
  <div className="card kpi-card">
    <div className="kpi-label">{label}</div>
    <div className={`kpi-value ${accent === 'warn' ? 'kpi-value-warn' : ''}`}>{value}</div>
    {sublabel && <div className="kpi-sublabel">{sublabel}</div>}
  </div>
);
