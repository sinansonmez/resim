import React from 'react';

import { cx, mutedPanelClass, panelClass, pillClass, primaryButtonClass } from './ui.js';

const accentBackgrounds = ['preset-card--accent-a', 'preset-card--accent-b', 'preset-card--accent-c'];

export function PresetLibrary({
  quickPresets,
  fullPresets,
  selectedPreset,
  selectAndApplyPreset,
  activePreset,
}) {
  return (
    <section className={cx(panelClass, 'preset-library reveal reveal--delay-2')}>
      <div className="preset-library__header">
        <div className="preset-library__pills">
          <span className={pillClass}>Preset desk</span>
          {activePreset && <span className="pill pill--accent">Live look</span>}
        </div>
        <h2 className="section-title">Choose the look</h2>
        <p className="section-copy">
          Quick looks are the fast lane. The full list behaves like a compact lookbook.
        </p>
      </div>

      {activePreset && (
        <div className={cx(mutedPanelClass, 'preset-library__active')}>
          <div className="preset-card__header">
            <div className="preset-card__copy">
              <strong className="preset-card__title">{activePreset.label}</strong>
              <span className="preset-card__body">{activePreset.style}</span>
            </div>
            <span className="preset-card__tag preset-card__tag--selected">
              {activePreset.inspiredBy}
            </span>
          </div>
          <span className="preset-card__body">{activePreset.bestFor}</span>
        </div>
      )}

      <div className="preset-library__quick">
        {quickPresets.map((preset) => (
          <button
            className={cx(
              primaryButtonClass,
              'quick-preset',
              preset.id === selectedPreset && 'quick-preset--selected'
            )}
            key={preset.id}
            onClick={() => selectAndApplyPreset(preset.id)}
            type="button"
          >
            {preset.label}
          </button>
        ))}
      </div>

      <div className="preset-library__list">
        {fullPresets.map((preset, index) => {
          const isSelected = preset.id === selectedPreset;
          const accentBackground = accentBackgrounds[index % accentBackgrounds.length];

          return (
            <button
              className={cx(
                'preset-card',
                accentBackground,
                isSelected && 'preset-card--selected'
              )}
              key={preset.id}
              onClick={() => selectAndApplyPreset(preset.id)}
              type="button"
            >
              <div className="preset-card__header">
                <strong className="preset-card__title">{preset.label}</strong>
                <span
                  className={cx('preset-card__tag', isSelected && 'preset-card__tag--selected')}
                >
                  {preset.inspiredBy}
                </span>
              </div>
              <span className="preset-card__body">{preset.style}</span>
              <div className="preset-card__chips">
                <span className="preset-card__chip">{preset.bestFor}</span>
                <span className="preset-card__chip preset-card__chip--muted">
                  Strength {preset.settings.filterStrength}
                </span>
              </div>
            </button>
          );
        })}
      </div>
    </section>
  );
}
