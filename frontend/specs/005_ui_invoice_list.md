# Spezifikation: Frontend Invoice List Component

## 1. Übersicht
Eine React-Komponente, die eine Liste von Rechnungen darstellt. Sie greift im Problem Space auf abgeleitete Status wie "Open", "Paid" zurück und muss diese visuell für den Geschäftskunden verständlich taggen.

## 2. Architektur Frontend
- Funktionales React-Setup (Vite).
- Component: `InvoiceList.tsx`
- Die Logik des Fetching wird abstrahiert (hier als statisches Array für den ersten Spec).
- Wir fokussieren uns erst auf TypeScript Interfaces, die zum Backend (Problemraum) identisch passen.

## 3. Testing Obligations
Später sollen via Vitest/Testing-Library Unit-Tests geschrieben werden, die sicherstellen, dass Overpaid Rechnungen in roter/warnender Farbe dargestellt werden, Paid in grün.