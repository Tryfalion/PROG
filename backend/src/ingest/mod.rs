//! Ingest-Schicht (Spec 005): wandelt rohe Datei-Uploads (JSON-Byte-Arrays) in unsere
//! Domänen-Typen um. Bewusst getrennt von der API-Schicht, damit das Parsen von Dateien
//! unabhängig von HTTP getestet werden kann (reine Funktionen, keine Axum-Abhängigkeit).

pub mod invoices_parser;
pub mod transactions_parser;
