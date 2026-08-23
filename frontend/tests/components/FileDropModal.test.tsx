import { describe, expect, it, vi } from 'vitest';
import { render, screen, fireEvent, waitFor } from '@testing-library/react';
import { FileDropModal } from '../../src/components/FileDropModal';
import * as api from '../../src/services/api';

describe('FileDropModal', () => {
  it('shows an error and skips the upload for a non-json file', async () => {
    render(<FileDropModal kind="invoice" onClose={() => {}} onUploaded={() => {}} />);

    const input = screen.getByLabelText('Upload file') as HTMLInputElement;
    const file = new File(['not json'], 'invoices.csv', { type: 'text/csv' });
    fireEvent.change(input, { target: { files: [file] } });

    await waitFor(() => {
      expect(screen.getByText(/Nur \.json Dateien/)).toBeInTheDocument();
    });
  });

  it('uploads a valid json file and shows the result summary', async () => {
    const uploadSpy = vi
      .spyOn(api, 'uploadFile')
      .mockResolvedValue({ inserted: 2, skipped: 1, errors: ['RE-3: unbekannte Währung'] });

    render(<FileDropModal kind="invoice" onClose={() => {}} onUploaded={() => {}} />);

    const input = screen.getByLabelText('Upload file') as HTMLInputElement;
    const file = new File(['[]'], 'invoices.json', { type: 'application/json' });
    fireEvent.change(input, { target: { files: [file] } });

    await waitFor(() => {
      expect(screen.getByText(/erfolgreich importiert/)).toBeInTheDocument();
    });
    expect(uploadSpy).toHaveBeenCalledWith('invoice', file);
  });
});
