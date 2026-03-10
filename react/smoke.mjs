import assert from 'node:assert/strict';

import {
  createDefaultControls,
  createTransformCatalog,
  formatControlValue,
} from './demo-utils.mjs';

const sampleCatalog = [
  {
    id: 'brightness',
    label: 'Brightness',
    method: 'brightnessImageData',
    control: {
      defaultValue: 30,
      display: 'signed',
    },
  },
  {
    id: 'opacity',
    label: 'Opacity',
    method: 'opacityImageData',
    control: {
      defaultValue: 100,
      display: 'percent',
    },
  },
  {
    id: 'sepia',
    label: 'Sepia',
    method: 'sepiaImageData',
  },
];

const catalog = createTransformCatalog(sampleCatalog);
const defaults = createDefaultControls(sampleCatalog);

assert.equal(catalog.brightness.label, 'Brightness');
assert.equal(catalog.opacity.method, 'opacityImageData');
assert.deepEqual(defaults, {
  brightness: 30,
  opacity: 100,
});
assert.equal(formatControlValue({ display: 'signed' }, -5), '-5');
assert.equal(formatControlValue({ display: 'signedPercent' }, 15), '+15%');
assert.equal(formatControlValue({ display: 'percent' }, 80), '80%');
assert.equal(formatControlValue({ display: 'float' }, 1.2), '1.2');
