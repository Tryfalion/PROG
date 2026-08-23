# Spec 009 — Abgleich-Dashboard (Reconciliation Board Redesign)

## Kontext
Die bestehenden Seiten `Statistics.tsx` und `LiveProcess.tsx` nutzen Mock-Daten und ein einfaches
Tabellen-Layout. Das Referenz-Design ("Invoice & Payments Comparison") zeigt ein dreispaltiges
Layout: Rechnungs-Ledger, Zahlungs-Ledger und ein "Reconciliation Board", das den Abgleich pro
Rechnung/Zahlung visualisiert (Perfect Match / Underpayment / Overdue), plus eine KPI-Kachelzeile
oben (Gesamtrechnungsbetrag, erhaltene Zahlungen, Matched-Quote, offene Diskrepanzen).

## Ziel / Scope
1. `pages/LiveProcess.tsx` wird zum Abgleich-Dashboard umgebaut:
   - KPI-Kachelzeile (4 Karten): Gesamt Invoiced, Deposits Received, Matched Collection Rate,
     Open Discrepancies — berechnet aus den geladenen Rechnungen/Transaktionen/Allocations.
   - Spalte 1 "Invoices Ledger": Liste aller Rechnungen mit Suchfeld, "+ Add Invoice" Button.
   - Spalte 2 "Payments Ledger" darunter oder daneben: Liste aller Transaktionen, "+ Add Payment".
   - Spalte 3 "Reconciliation Board": pro Rechnung eine Karte mit Status-Badge
     (Perfect Match / Underpayment / Overdue, abgeleitet aus `InvoiceStatus`), Betrag, verknüpfter
     Zahlung(en) und "Unresolved Balance" (Rechnungsbetrag − Summe zugewiesener Zahlungen).
2. `pages/Statistics.tsx` bleibt als separate Detailstatistik-Seite bestehen, bezieht die Zahlen
   aber real aus `hooks/useInvoicesAndTransactions` statt aus hartcodierten Mocks.
3. Farb-/Layout-Konventionen aus Spec 008 (Perlweiss `#FDFDFD`, Blau `#0A4B8F`) werden beibehalten
   und um Karten-Schatten/Rundungen ergänzt, die dem Referenzbild entsprechen.

## Verhalten & Regeln
- "Matched Collection Rate" = (Anzahl Rechnungen mit Status `Paid` oder `Overpaid`) / (Gesamtzahl
  Rechnungen), als Prozentwert gerundet.
- "Open Discrepancies" = Anzahl Rechnungen, deren Status nicht `Paid` ist (`Open` oder `Overpaid`
  gelten beide als abzuklärende Diskrepanz für die Kachel-Anzeige).
- Reconciliation-Karten sind rein lesend (kein manuelles Umsortieren); Klick auf eine Karte
  markiert sie nur visuell als ausgewählt (kein Popup nötig für V1).

## Testpflichten
- `pages/LiveProcess.test.tsx`:
  - KPI "Open Discrepancies" zeigt korrekten Wert bei gemischten Rechnungsstatus.
  - Reconciliation-Karte für eine überzahlte Rechnung zeigt Status-Badge "Overpaid".
  - Suchfeld filtert die Invoices-Ledger-Liste nach Rechnungsnummer.
