import { describe, expect, it } from 'vitest';
import { render, screen } from '@testing-library/react';
import { KpiCard } from '../../src/components/KpiCard';

describe('KpiCard', () => {
  it('renders label, value and sublabel', () => {
    render(<KpiCard label="Open Discrepancies" value="5" sublabel="Unresolved" accent="warn" />);
    expect(screen.getByText('Open Discrepancies')).toBeInTheDocument();
    expect(screen.getByText('5')).toBeInTheDocument();
    expect(screen.getByText('Unresolved')).toBeInTheDocument();
  });
});
