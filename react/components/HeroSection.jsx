import React from 'react';

import {
  cx,
  darkPanelClass,
  mutedPanelClass,
  panelClass,
  pillClass,
  primaryButtonClass,
} from './ui.js';

function SessionMetric({ label, value }) {
  return (
    <div className="metric-card">
      <div className="metric-card__label">{label}</div>
      <strong className="metric-card__value">{value}</strong>
    </div>
  );
}

export function HeroSection({
  activePreset,
  historyDepth,
  isReady,
  onUploadClick,
  presetCount,
  quickPresetCount,
  sourceLabel,
}) {
  return (
    <section className="hero">
      <div className={cx(darkPanelClass, 'hero__main reveal')}>
        <div className="hero__glow hero__glow--blue" />
        <div className="hero__glow hero__glow--green" />

        <div className="hero__intro">
          <span className="hero__eyebrow">Resim Studio</span>
          <div className="hero__copy-block">
            <h1 className="hero__title">
              Fast looks,
              <br />
              cleaner control.
            </h1>
            <p className="hero__copy">
              The demo now behaves like a compact edit studio: start from the original,
              apply a look in one click, and move into manual adjustments only when the
              image needs a second pass.
            </p>
          </div>
        </div>

        <div className="hero__actions">
          <button
            className={cx(
              primaryButtonClass,
              'button--large'
            )}
            onClick={onUploadClick}
            type="button"
          >
            Upload image
          </button>
        </div>

        <div className="hero__metrics">
          <SessionMetric label="Live preset" value={activePreset?.label ?? 'No preset'} />
          <SessionMetric label="Quick looks" value={`${quickPresetCount} ready`} />
          <SessionMetric label="History" value={`${historyDepth} steps`} />
        </div>
      </div>

      <div className={cx(panelClass, 'hero__aside reveal reveal--delay-1')}>
        <div className="hero__aside-top">
          <span className={pillClass}>Live session</span>
          <span
            className={cx(
              pillClass,
              isReady ? 'pill--success' : 'pill--warning'
            )}
          >
            {isReady ? 'Ready' : 'Loading'}
          </span>
        </div>

        <div className="hero__aside-copy">
          <h2 className="hero__aside-title">{activePreset?.label}</h2>
          <p className="hero__aside-text">
            Inspired by {activePreset?.inspiredBy}. Best for {activePreset?.bestFor}.
          </p>
        </div>

        <div className={cx(mutedPanelClass, 'hero__summary')}>
          <div className="hero__detail">
            <div className="hero__detail-label">Current source</div>
            <strong className="hero__detail-value">{sourceLabel}</strong>
          </div>
          <div className="hero__detail">
            <div className="hero__detail-label">Look summary</div>
            <span className="hero__detail-text">{activePreset?.style}</span>
          </div>
          <div className="hero__detail">
            <div className="hero__detail-label">Preset strength</div>
            <span className="hero__detail-text">{activePreset?.settings.filterStrength}</span>
          </div>
        </div>

        <div className="hero__fact-grid">
          <div className={cx(mutedPanelClass, 'hero__fact')}>
            <div className="hero__detail-label">Library</div>
            <strong className="hero__fact-value">{presetCount} looks ready</strong>
          </div>
          <div className={cx(mutedPanelClass, 'hero__fact')}>
            <div className="hero__detail-label">Workflow</div>
            <strong className="hero__fact-value">Original stays intact</strong>
          </div>
        </div>
      </div>
    </section>
  );
}
