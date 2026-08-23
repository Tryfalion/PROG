import React, { useState } from 'react';
import { uploadFile } from '../services/api';
import type { UploadResult } from '../services/types';

interface Props {
  kind: 'invoice' | 'payment';
  onClose: () => void;
  onUploaded: (result: UploadResult) => void;
}

/**
 * Modal mit Drag & Drop Zone zum Hochladen einer JSON-Datei mit Rechnungen oder Zahlungen
 * (Spec 007). Unterstützt zusätzlich einen klassischen Datei-Auswahl-Dialog per Klick.
 */
export const FileDropModal: React.FC<Props> = ({ kind, onClose, onUploaded }) => {
  const [isDragging, setIsDragging] = useState(false);
  const [isUploading, setIsUploading] = useState(false);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [result, setResult] = useState<UploadResult | null>(null);

  const title = kind === 'invoice' ? 'Add Invoice' : 'Add Payment';

  const handleFile = async (file: File) => {
    if (!file.name.toLowerCase().endsWith('.json')) {
      setErrorMessage('Nur .json Dateien werden unterstützt.');
      return;
    }
    setErrorMessage(null);
    setIsUploading(true);
    try {
      const uploadResult = await uploadFile(kind, file);
      setResult(uploadResult);
      onUploaded(uploadResult);
    } catch (error) {
      setErrorMessage(`Upload fehlgeschlagen: ${String(error)}`);
    } finally {
      setIsUploading(false);
    }
  };

  const onDrop: React.DragEventHandler<HTMLLabelElement> = (event) => {
    event.preventDefault();
    setIsDragging(false);
    if (isUploading) return;

    const files = event.dataTransfer.files;
    if (files.length > 1) {
      setErrorMessage('Nur eine Datei gleichzeitig wird unterstützt.');
      return;
    }
    if (files.length === 1) {
      void handleFile(files[0]);
    }
  };

  const onFileInputChange: React.ChangeEventHandler<HTMLInputElement> = (event) => {
    const file = event.target.files?.[0];
    if (file) void handleFile(file);
  };

  return (
    <div className="modal-overlay" role="dialog" aria-modal="true" aria-label={title}>
      <div className="modal-card">
        <div className="modal-header">
          <h3>{title}</h3>
          <button className="modal-close" onClick={onClose} aria-label="Schließen">
            ×
          </button>
        </div>

        {!result && (
          <label
            className={`dropzone ${isDragging ? 'dropzone-active' : ''} ${isUploading ? 'dropzone-disabled' : ''}`}
            onDragOver={(e) => {
              e.preventDefault();
              if (!isUploading) setIsDragging(true);
            }}
            onDragLeave={() => setIsDragging(false)}
            onDrop={onDrop}
          >
            <input
              type="file"
              accept=".json,application/json"
              aria-label="Upload file"
              onChange={onFileInputChange}
              disabled={isUploading}
              style={{ display: 'none' }}
            />
            <p>{isUploading ? 'Datei wird hochgeladen…' : 'Datei hierher ziehen oder klicken zum Auswählen'}</p>
            <p className="dropzone-hint">Erwartet: JSON-Array ({kind === 'invoice' ? 'Rechnungen' : 'Zahlungen'})</p>
          </label>
        )}

        {errorMessage && <p className="upload-error">{errorMessage}</p>}

        {result && (
          <div className="upload-summary">
            <p>
              <b>{result.inserted}</b> erfolgreich importiert, <b>{result.skipped}</b> übersprungen.
            </p>
            {result.errors.length > 0 && (
              <ul>
                {result.errors.map((err, idx) => (
                  <li key={idx}>{err}</li>
                ))}
              </ul>
            )}
            <button className="btn-primary" onClick={onClose}>
              Schließen
            </button>
          </div>
        )}
      </div>
    </div>
  );
};
