export function createTransformCatalog(entries) {
  return Object.fromEntries(entries.map((entry) => [entry.id, entry]));
}

export function createDefaultControls(entries) {
  return Object.fromEntries(
    entries
      .filter((transform) => transform.control)
      .map((transform) => [transform.id, transform.control.defaultValue])
  );
}

export function formatControlValue(control, value) {
  switch (control.display) {
    case 'signed':
      return `${value > 0 ? '+' : ''}${value}`;
    case 'signedPercent':
      return `${value > 0 ? '+' : ''}${value}%`;
    case 'percent':
      return `${value}%`;
    case 'float':
      return Number(value).toFixed(1);
    default:
      return `${value}`;
  }
}
