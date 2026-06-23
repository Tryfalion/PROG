# Spezifikation: UI & Dashboard (React)

## 1. Übersicht
Das Frontend erhält ein professionelles Theme:
- Hauptfarbe: **Perlweiss (#FDFDFD / #F5F5F5)** für den Hintergrund
- Akzentfarbe: **Kräftiges Blau (#0A4B8F / #1E3A8A)** für Header und Interaktionselemente.

## 2. Masken (Views)
### Maske 1: Statstiken (`Statistics.tsx`)
Zeigt KPI-Karten (ähnlich einem Management-Dashboard):
- Anzahl Rechnungen / Zahlungen
- Aufteilung: Bezahlt vs. Offen
- Summen: Eingenommener Betrag vs. Ausstehender Betrag

### Maske 2: Live-Prozess (`LiveProcess.tsx`)
Inbox/Stream-Ansicht:
- Links: Die Rechnungen, die hereinkommen.
- Rechts: Die Banktransaktionen.
- Highlight: Farbliche Status Tags basierend auf den N2 Views.
  - Grün: `Paid` (Bezahlt)
  - Rot: `Open` (Nicht bezahlt)
  - Gelb: `Overpaid` (Überbezahlt / Teilbezahlt)

## 3. Testing Obligations
- Keine direkten Unit-Tests hier angelegt, da Vitest Setup komplexer ist, aber die Props & Props-Destrukturierung muss strikt via DOM-Hierarchie gekapselt sein.