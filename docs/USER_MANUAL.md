# LedgerGate — Nutzerhandbuch

Dieses Handbuch richtet sich an Endnutzer:innen von LedgerGate (Solo-Selbstständige,
kleine Teams), die Rechnungen und Zahlungen abgleichen möchten.

## 1. Anmeldung & Oberfläche

Nach dem Start ist die Weboberfläche unter `http://localhost:5173` (Entwicklungsmodus) bzw.
der vom Administrator konfigurierten Adresse erreichbar. Die Navigation oben bietet drei Bereiche:

- **Statistiken** — Kennzahlen-Übersicht (Rechnungs-/Zahlungsvolumen, offene Beträge).
- **Abgleich** — das zentrale "Invoice & Payments Comparison" Dashboard (Reconciliation Board).
- **Einstellungen** — zeigt die aktuell verwendete Backend-Adresse an.

## 2. Rechnungen und Zahlungen importieren (Drag & Drop)

Im Bereich **Abgleich** gibt es zwei Schaltflächen:

- **+ Add Invoice** über der "Invoices Ledger"-Liste.
- **+ Add Payment** über der "Payments Ledger"-Liste.

Beim Klick öffnet sich ein Fenster mit einer Ablagefläche (Drop-Zone):

1. Ziehen Sie eine `.json`-Datei per Drag & Drop auf die gestrichelte Fläche, **oder**
2. klicken Sie auf die Fläche, um einen Datei-Auswahl-Dialog zu öffnen.

Die Datei muss ein JSON-Array enthalten. Beispiel für Rechnungen:

```json
[
  {"invoice_number": "RE-2026-001", "issue_date": "2026-06-01", "due_date": "2026-06-14", "amount": "150.00", "currency": "EUR"}
]
```

Beispiel für Zahlungen (Banktransaktionen):

```json
[
  {"booking_date": "2026-06-05", "value_date": "2026-06-05", "amount": "150.00", "currency": "EUR", "reference_text": "Zahlung für RE-2026-001", "counterparty_name": "Max Mustermann"}
]
```

Nach dem Upload zeigt das Fenster an, wie viele Einträge erfolgreich importiert (`inserted`) und
wie viele übersprungen wurden (`skipped`, z. B. wegen unbekannter Währung oder doppelter
Rechnungsnummer). Fehlerhafte Einzeleinträge stoppen nicht den gesamten Import.

> Beispieldateien zum Ausprobieren finden Sie unter `test_data/invoices_mock.json` und
> `test_data/transactions_mock.json` im Projektverzeichnis.

## 3. Automatischer Abgleich (Reconciliation)

Sobald eine Zahlung importiert wird, versucht das System automatisch, sie einer offenen Rechnung
zuzuordnen — anhand der Rechnungsnummer im Verwendungszweck. Das Ergebnis erscheint sofort
(live, ohne Neuladen der Seite) im **Reconciliation Board**:

| Badge | Bedeutung |
|---|---|
| **Perfect Match** (grün) | Rechnung wurde exakt bezahlt. |
| **Overpayment** (gelb) | Es wurde mehr bezahlt als in Rechnung gestellt — manuelle Prüfung empfohlen. |
| **Open / Unpaid** (rot) | Keine passende Zahlung gefunden. |

## 4. Kennzahlen (KPIs)

Über dem Board zeigen vier Kacheln:

- **Total Invoiced** — Summe aller Rechnungsbeträge.
- **Deposits Received** — Summe aller importierten Zahlungen.
- **Matched Collection Rate** — Anteil der (über-)bezahlten Rechnungen in Prozent.
- **Open Discrepancies** — Anzahl der Rechnungen, die noch nicht vollständig bezahlt sind.

## 5. Suche

Im "Invoices Ledger" kann über das Suchfeld nach Rechnungsnummern gefiltert werden.

## 6. Fehlerbehebung

- **"Verbindung zum Backend fehlgeschlagen"**: Das Backend ist nicht erreichbar. Prüfen Sie, ob
  der Server läuft (siehe Administrator-Handbuch, `docs/ADMIN_SETUP.md`).
- **Datei wird nicht akzeptiert**: Es werden ausschließlich `.json`-Dateien unterstützt.
- **Zahlung wird keiner Rechnung zugeordnet**: Der Verwendungszweck muss die exakte
  Rechnungsnummer enthalten (z. B. "Zahlung für RE-2026-001").
