import React from 'react';
import { BrowserRouter, Routes, Route, Link } from 'react-router-dom';
import { Statistics } from './pages/Statistics';
import { LiveProcess } from './pages/LiveProcess';
import { Settings } from './pages/Settings';
import './index.css';

export const App: React.FC = () => {
  return (
    <BrowserRouter>
      <nav className="dashboard-nav">
        <div style={{ color: 'white', marginRight: '30px', fontWeight: '900' }}>LedgerGate</div>
        <Link to="/">Statistiken</Link>
        <Link to="/live">Abgleich</Link>
        <Link to="/settings">Einstellungen</Link>
      </nav>
      
      <div style={{ padding: '20px' }}>
        <Routes>
          <Route path="/" element={<Statistics />} />
          <Route path="/live" element={<LiveProcess />} />
          <Route path="/settings" element={<Settings />} />
        </Routes>
      </div>
    </BrowserRouter>
  );
};
