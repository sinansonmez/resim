import React from 'react';

import {
  cx,
  mutedPanelClass,
  panelClass,
  pillClass,
  primaryButtonClass,
  secondaryButtonClass,
} from './ui.js';

export function SourcePanel({ isReady, sourceLabel, onUploadClick, restoreSampleImage }) {
  return (
    <section className={cx(panelClass, 'source-panel reveal reveal--delay-1')}>
      <div className="source-panel__header">
        <div className="source-panel__headline">
          <span className={pillClass}>Input</span>
          <h2 className="section-title section-title--compact">Source image</h2>
        </div>
        <span className={cx(pillClass, isReady ? 'pill--success' : 'pill--warning')}>
          {isReady ? 'Loaded' : 'Waiting'}
        </span>
      </div>

      <p className="section-copy">
        Use your own photo or fall back to the bundled sample when you want to inspect the
        preset library quickly.
      </p>

      <div className={cx(mutedPanelClass, 'source-panel__details')}>
        <div className="source-panel__label">Current source</div>
        <strong className="source-panel__value">{sourceLabel}</strong>
        <span className="source-panel__copy">
          All preset applications restart from the original file.
        </span>
      </div>

      <div className="source-panel__actions">
        <button className={cx(primaryButtonClass)} onClick={onUploadClick} type="button">
          Upload image
        </button>
        <button className={cx(secondaryButtonClass)} onClick={restoreSampleImage} type="button">
          Use sample
        </button>
      </div>
    </section>
  );
}
