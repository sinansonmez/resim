import React, { useEffect, useRef } from 'react';

import { formatControlValue } from '../demo-utils.mjs';
import {
  cx,
  mutedPanelClass,
  panelClass,
  pillClass,
  primaryButtonClass,
  secondaryButtonClass,
} from './ui.js';

function TransformSelector({ transformEntries, selectedTransform, setSelectedTransform }) {
  return (
    <div className={cx(mutedPanelClass, 'inspector__card')}>
      <div className="inspector__card-header">
        <strong className="inspector__label">Manual tools</strong>
        <span className="inspector__hint">{transformEntries.length} transforms</span>
      </div>
      <div className="tool-grid">
        {transformEntries.map(([key, transform]) => {
          const isSelected = key === selectedTransform;

          return (
            <button
              className={cx('tool-button', isSelected && 'tool-button--selected')}
              key={key}
              onClick={() => setSelectedTransform(key)}
              type="button"
            >
              <strong className="tool-button__title">{transform.label}</strong>
              <span className="tool-button__body">{transform.description}</span>
            </button>
          );
        })}
      </div>
    </div>
  );
}

function ControlPanel({ selectedConfig, selectedControl, selectedControlValue, updateControlValue }) {
  return (
    <div className={cx(mutedPanelClass, 'inspector__card')}>
      <div className="control-card__copy">
        <strong className="control-card__title">{selectedConfig?.label ?? 'Transform control'}</strong>
        <span className="control-card__body">{selectedConfig?.description}</span>
      </div>
      {selectedControl ? (
        <>
          <div className="control-card__meta">
            <strong className="control-card__label">{selectedControl.label}</strong>
            <code className="control-card__value">
              {formatControlValue(selectedControl, selectedControlValue)}
            </code>
          </div>
          <input
            className="control-card__range"
            max={selectedControl.max}
            min={selectedControl.min}
            onChange={updateControlValue}
            step={selectedControl.step}
            type="range"
            value={selectedControlValue}
          />
          <div className="control-card__range-meta">
            <span>{selectedControl.min}</span>
            <span>{selectedControl.max}</span>
          </div>
        </>
      ) : (
        <p className="control-card__body">
          This transform applies with its built-in settings. Use the action button below to
          run it on the current preview.
        </p>
      )}
    </div>
  );
}

export function FineTunePanel({
  isFineTuneOpen,
  setIsFineTuneOpen,
  transformEntries,
  selectedTransform,
  setSelectedTransform,
  selectedControl,
  selectedControlValue,
  updateControlValue,
  applySelectedTransform,
  selectedConfig,
  resizePreview,
  isReady,
  resizeWidth,
  setResizeWidth,
  resizeHeight,
  setResizeHeight,
  sourceLabel,
  historyDepth,
}) {
  const panelRef = useRef(null);

  useEffect(() => {
    if (!isFineTuneOpen) {
      return;
    }

    panelRef.current?.scrollIntoView({
      behavior: 'smooth',
      block: 'start',
    });
  }, [isFineTuneOpen]);

  return (
    <section
      className={cx(panelClass, 'inspector reveal reveal--delay-2', isFineTuneOpen && 'inspector--open')}
      ref={panelRef}
    >
      <div className="inspector__header">
        <div className="inspector__headline">
          <div className="inspector__pills">
            <span className={pillClass}>Fine tune</span>
            {selectedConfig && <span className="pill pill--accent">{selectedConfig.label}</span>}
          </div>
          <h3 className="section-title section-title--compact">Inspector controls</h3>
          <span className="section-copy section-copy--compact">
            Use them after a preset when the image needs a second pass.
          </span>
        </div>
        <button
          className={cx(secondaryButtonClass, 'button--compact')}
          onClick={() => setIsFineTuneOpen((current) => !current)}
          type="button"
        >
          {isFineTuneOpen ? 'Collapse' : 'Expand'}
        </button>
      </div>

      <div className={cx(mutedPanelClass, 'inspector__summary')}>
        <div className="inspector__summary-row">
          <div>
            <div className="inspector__label">Source</div>
            <strong className="inspector__value">{sourceLabel}</strong>
          </div>
          <div className="inspector__summary-side">
            <div className="inspector__label">Undo stack</div>
            <strong className="inspector__value">{historyDepth}</strong>
          </div>
        </div>
      </div>

      {isFineTuneOpen && (
        <div className="inspector__body">
          <TransformSelector
            selectedTransform={selectedTransform}
            setSelectedTransform={setSelectedTransform}
            transformEntries={transformEntries}
          />

          <ControlPanel
            selectedConfig={selectedConfig}
            selectedControl={selectedControl}
            selectedControlValue={selectedControlValue}
            updateControlValue={updateControlValue}
          />

          <button
            className={cx(primaryButtonClass, 'button--block')}
            disabled={!isReady || !selectedConfig}
            onClick={applySelectedTransform}
            type="button"
          >
            Apply {selectedConfig ? selectedConfig.label : 'Transform'}
          </button>

          <div className={cx(mutedPanelClass, 'inspector__card')}>
            <div className="inspector__card-header">
              <strong className="inspector__label">Utility</strong>
              <span className="inspector__hint">Resize current output</span>
            </div>
            <div className="field-grid">
              <label className="field">
                <span className="field__label">Width</span>
                <input
                  className="field__input"
                  min="1"
                  onChange={(event) => setResizeWidth(event.target.value)}
                  type="number"
                  value={resizeWidth}
                />
              </label>
              <label className="field">
                <span className="field__label">Height</span>
                <input
                  className="field__input"
                  min="1"
                  onChange={(event) => setResizeHeight(event.target.value)}
                  type="number"
                  value={resizeHeight}
                />
              </label>
            </div>
            <button
              className={cx(secondaryButtonClass, 'button--dark button--block')}
              disabled={!isReady}
              onClick={resizePreview}
              type="button"
            >
              Resize canvas
            </button>
          </div>
        </div>
      )}
    </section>
  );
}
