import { useCallback, useEffect, useState } from 'react';
import { getInvoices, getTransactions } from '../services/api';
import type { InvoiceDto, TransactionDto } from '../services/types';
import { useWebSocket } from './useWebSocket';

/**
 * Lädt Rechnungen & Transaktionen initial per REST und hält sie über den WebSocket-Hook
 * aktuell: bei jedem Realtime-Event wird die betroffene Liste einfach neu geladen
 * ("refetch on event"), bewusst einfach gehalten (Spec 006).
 */
export function useInvoicesAndTransactions() {
  const [invoices, setInvoices] = useState<InvoiceDto[]>([]);
  const [transactions, setTransactions] = useState<TransactionDto[]>([]);
  const [error, setError] = useState<string | null>(null);

  const reloadInvoices = useCallback(() => {
    getInvoices()
      .then(setInvoices)
      .catch((e) => setError(String(e)));
  }, []);

  const reloadTransactions = useCallback(() => {
    getTransactions()
      .then(setTransactions)
      .catch((e) => setError(String(e)));
  }, []);

  useEffect(() => {
    reloadInvoices();
    reloadTransactions();
  }, [reloadInvoices, reloadTransactions]);

  useWebSocket((event) => {
    if (event.type === 'transaction_created') {
      reloadTransactions();
    } else {
      // "invoice_created" und "allocation_created" verändern beide den Status von Rechnungen.
      reloadInvoices();
    }
  });

  return { invoices, transactions, error, reloadInvoices, reloadTransactions };
}
