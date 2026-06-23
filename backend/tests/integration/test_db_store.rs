#[cfg(test)]
mod tests {
    // Dies ist ein Blueprint für den DB Store Integration Test.
    // Gemäß Specs 006 nutzt es `testcontainers` um einen echten Postgres
    // hochzufahren. (Die Umsetzung benötigt async Runtimes).
    
    // #[tokio::test]
    // async fn test_database_insert_and_view() {
    //    // 1. Container hochfahren
    //    // 2. sqlx::migrate! anwenden
    //    // 3. Rechnung via insert_invoice schreiben
    //    // 4. Zahlung via insert_transaction schreiben
    //    // 5. Allocation schreiben
    //    // 6. get_invoices_with_status abfragen
    //    // 7. assert_eq!(status, InvoiceStatus::Paid)
    // }
}
