use serde::Serialize;

use crate::problemraum::matching::Allocation;
use crate::problemraum::models::{BankTransaction, Invoice};

/// Sehr einfache Serialize-Wrapper für WS-Events (Spec 007), damit die reinen Domänen-Typen
/// (`problemraum::models`) selbst frei von Serde-Abhängigkeiten bleiben.
#[derive(Serialize)]
struct InvoiceEvent<'a> {
    #[serde(rename = "type")]
    event_type: &'static str,
    invoice_number: &'a str,
    amount: String,
}

#[derive(Serialize)]
struct TransactionEvent<'a> {
    #[serde(rename = "type")]
    event_type: &'static str,
    reference_text: &'a str,
    amount: String,
}

#[derive(Serialize)]
struct AllocationEvent {
    #[serde(rename = "type")]
    event_type: &'static str,
    invoice_id: String,
    transaction_id: String,
    allocated_amount: String,
}

/// Sendet ein Event an alle verbundenen WebSocket-Clients, nachdem eine Rechnung
/// erfolgreich gespeichert wurde.
pub fn notify_invoice_created(invoice: &Invoice) {
    let event = InvoiceEvent {
        event_type: "invoice_created",
        invoice_number: &invoice.invoice_number,
        amount: invoice.amount.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&event) {
        crate::ws::broadcast_event(json);
    }
}

/// Sendet ein Event, nachdem eine Banktransaktion erfolgreich gespeichert wurde.
pub fn notify_transaction_created(tx: &BankTransaction) {
    let event = TransactionEvent {
        event_type: "transaction_created",
        reference_text: &tx.reference_text,
        amount: tx.amount.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&event) {
        crate::ws::broadcast_event(json);
    }
}

/// Sendet ein Event, nachdem eine automatische oder manuelle Zuweisung erstellt wurde.
pub fn notify_allocation_created(alloc: &Allocation) {
    let event = AllocationEvent {
        event_type: "allocation_created",
        invoice_id: alloc.invoice_id.to_string(),
        transaction_id: alloc.transaction_id.to_string(),
        allocated_amount: alloc.allocated_amount.to_string(),
    };
    if let Ok(json) = serde_json::to_string(&event) {
        crate::ws::broadcast_event(json);
    }
}
