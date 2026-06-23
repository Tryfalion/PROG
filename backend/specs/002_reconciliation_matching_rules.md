# Spezifikation: Reconciliation & Matching Algorithmen (Problem Space)

## 1. Übersicht
Diese Spezifikation definiert das Herzstück des "Problemraums": Den Matching-Algorithmus und die Zuweisungs-Logik (Allocations). Da das System den Zustand nur durch N2-Fakten ableitet, müssen wir spezifizieren, wie eine Bezahlung auf einer Rechnung "landet".

## 2. Zuweisung (Allocation)
Eine Zuweisung ist der Link zwischen einer `BankTransaction` und einer `Invoice`.
Es können mehrere Zuweisungen auf eine Rechnung existieren (z. B. Teilzahlungen).

### 2.1 Modellierung
- `Allocation` (Struct):
  - `transaction_id`: Uuid
  - `invoice_id`: Uuid
  - `allocated_amount`: Decimal (Der Teiles der Transaktion, der dieser Rechnung zugeordnet wurde)

## 3. Matching-Regeln
Die Zuordnung erfolgt auf Basis des Verwendungszwecks (`reference_text`).

1. **Exaktes Matching:** Der `reference_text` der Banktransaktion enthält 1:1 die `invoice_number`.
2. **Betrags-Plausibilität:** Der `allocated_amount` ist primär der verfügbare Betrag der Transaktion. Ist die Transaktion höher als die Rechnung (Overpayment), wird der gesamte Transaktionsbetrag zur Markierung als Sonderfall (Overpaid) gebucht ODER nur der Rechnungsbetrag, und der Rest bleibt unzugewiesen (dies muss per Code-Kommentar als Entscheidung festgehalten werden -> Wir buchen alles drauf, Status wird "Overpaid").
3. **Währungskonflikt (Currency Mismatch):** Wenn die `BankTransaction` in USD ist, die `Invoice` in EUR, muss das System dies ablehnen oder als Sonderfall behandeln. Für V1: Wir erlauben Matches nur bei gleicher Währung, ansonsten Fallback auf manuelle Zuordnung.

## 4. Testing Obligations
- **Unit-Test 1:** Exaktes Zahlen einer Rechnung. Transaktion = 100€, Rechnung = 100€. Resultat: Voll zugewiesen.
- **Unit-Test 2:** Teilzahlung. Transaktion = 50€, Rechnung = 100€. Resultat: 50 zugewiesen, Rechnung bleibt `Open`.
- **Unit-Test 3:** Overpayment. Transaktion = 150€, Rechnung = 100€.
- **Unit-Test 4:** Währungs-Abweisung. Eine Zahlung in USD wirft einen sauberen Domain-Error beim Match auf eine EUR-Rechnung.

## 5. Freigabe
Bitte lies dir die neuen Problemraum-Regeln durch, vor allem bezüglich der Allocations und Teilzahlungen.
Warte auf Freigabe ("Go"), bevor der Code in Rust (`services/matching.rs` etc.) umgesetzt wird.