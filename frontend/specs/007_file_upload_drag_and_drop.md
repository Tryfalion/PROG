# Spec 007 — Drag & Drop Datei-Upload (Add Invoice / Add Payment)

## Kontext
Laut Referenz-Design ("Invoice & Payments Comparison") gibt es je eine Schaltfläche
"+ Add Invoice" und "+ Add Payment" über den jeweiligen Ledger-Listen. Diese sollen ein Fenster
(Modal) öffnen, in dem Dateien per Drag & Drop oder Datei-Auswahl-Dialog abgelegt werden können.

## Ziel / Scope
1. `components/FileDropModal.tsx` — Wiederverwendbares Modal mit einer Drop-Zone:
   - Reagiert auf native HTML5 Drag-Events (`onDragOver`, `onDragLeave`, `onDrop`).
   - Alternativ: Klick öffnet einen normalen `<input type="file">` Dialog.
   - Zeigt den Dateinamen sowie Upload-Status (lädt / Erfolg / Fehler) an.
   - Nimmt `kind: 'invoice' | 'payment'` und `onUploaded: (result: UploadResult) => void` als Props.
2. Integration in `LiveProcess.tsx` (bzw. der neuen Dashboard-Seite aus Spec 009): Klick auf
   "+ Add Invoice" öffnet das Modal mit `kind="invoice"`, "+ Add Payment" mit `kind="payment"`.
3. Beim Drop/Auswahl wird die Datei per `multipart/form-data` an
   `POST /api/v1/invoices/upload` bzw. `POST /api/v1/transactions/upload` gesendet
   (`services/api.ts`, Spec 006).
4. Nach erfolgreichem Upload zeigt das Modal eine Zusammenfassung (`inserted`/`skipped`/`errors`)
   und schließt sich nach Bestätigung; die Listen werden über das WebSocket-Event automatisch
   aktualisiert (kein manueller Reload nötig).

## Verhalten & Regeln
- Nur `.json` Dateien werden akzeptiert (clientseitige Prüfung der Dateiendung als erste,
  einfache Hürde; die eigentliche Validierung erfolgt serverseitig, Spec 005).
- Mehrfach-Drop (mehrere Dateien gleichzeitig): nur die erste Datei wird verarbeitet, mit
  Hinweistext "Nur eine Datei gleichzeitig wird unterstützt".
- Während des Uploads ist die Drop-Zone deaktiviert (kein Doppel-Upload möglich).

## Testpflichten
- `components/FileDropModal.test.tsx`:
  - Drop-Event mit einer Datei löst den Upload-Aufruf mit korrektem `kind`-Endpunkt aus (gemockt).
  - Ungültige Dateiendung zeigt Fehlermeldung, kein Upload-Aufruf.
  - Erfolgreicher Upload zeigt die Zusammenfassung mit `inserted`/`skipped` an.
