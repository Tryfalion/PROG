import React from 'react';
import ReactDOM from 'react-dom/client';
import { App } from './App';

// Einstiegspunkt der React-Anwendung (siehe Spec 008/009 für die Seiten-Struktur).
ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
