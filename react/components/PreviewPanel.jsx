import React from 'react';

import {
  cx,
  mutedPanelClass,
  panelClass,
  pillClass,
  primaryButtonClass,
  secondaryButtonClass,
  subtleButtonClass,
} from './ui.js';

export function PreviewPanel({
  activePreset,
  canvasRef,
  originalCanvasRef,
  imageRef,
  sourceImageUrl,
  sourceLabel,
  isComparisonMode,
  setIsComparisonMode,
  historyDepth,
  isReady,
  isFineTuneOpen,
  setIsFineTuneOpen,
  undoLastStep,
  resetPreview,
  downloadPreview,
  status,
  syncCanvasWithSource,
}) {
  return (
    <section className={cx(panelClass, 'preview reveal reveal--delay-1')}>
      <div className="preview__content">
        <div className="preview__header">
          <div className="preview__headline">
            <div className="preview__pills">
              <span className={pillClass}>Workspace</span>
              <span className={pillClass}>{sourceLabel}</span>
            </div>
            <h2 className="preview__title">Live preview</h2>
            <p className="preview__copy">
              {activePreset
                ? `${activePreset.label} is active. Compare it against the original, then export or move into fine tuning.`
                : 'Keep the image large, keep the actions close, and inspect every change against the original when needed.'}
            </p>
          </div>
          <label className="compare-toggle">
            <span className="compare-toggle__label">Compare original</span>
            <span className="switch">
              <input
                checked={isComparisonMode}
                className="switch__input"
                onChange={(event) => setIsComparisonMode(event.target.checked)}
                type="checkbox"
              />
              <span className="switch__track" />
              <span className="switch__thumb" />
            </span>
          </label>
        </div>

        <div className="preview__toolbar">
          <button
            className={cx(subtleButtonClass, 'preview__tool')}
            disabled={historyDepth === 0}
            onClick={undoLastStep}
            type="button"
          >
            Undo
          </button>
          <button
            className={cx(secondaryButtonClass, 'preview__tool')}
            disabled={!isReady}
            onClick={resetPreview}
            type="button"
          >
            Reset
          </button>
          <button
            className={cx(primaryButtonClass, 'preview__tool')}
            disabled={!isReady}
            onClick={downloadPreview}
            type="button"
          >
            Export PNG
          </button>
          <button
            className={cx(secondaryButtonClass, 'button--soft preview__tool')}
            onClick={() => setIsFineTuneOpen((current) => !current)}
            type="button"
          >
            {isFineTuneOpen ? 'Hide fine tune' : 'Open fine tune'}
          </button>
        </div>

        <div className="stage">
          <div className={cx('stage__grid', isComparisonMode && 'stage__grid--compare')}>
            {isComparisonMode && (
              <div className="stage__column">
                <div className="stage__meta">
                  <strong>Original</strong>
                  <span>Baseline</span>
                </div>
                <div className="stage__outer">
                  <div className="stage__canvas-frame">
                    <canvas className="stage__canvas" ref={originalCanvasRef} />
                  </div>
                </div>
              </div>
            )}
            <div className="stage__column">
              <div className="stage__meta">
                <strong>{isComparisonMode ? 'Processed' : 'Preview'}</strong>
                <span>{activePreset?.label ?? 'Current look'}</span>
              </div>
              <div className="stage__outer">
                <div className="stage__canvas-frame">
                  <canvas className="stage__canvas" ref={canvasRef} />
                </div>
              </div>
            </div>
          </div>
        </div>

        <div className="preview__footer">
          <div className={cx(mutedPanelClass, 'preview__status')}>
            <div className="preview__status-top">
              <span className="preview__label">Status</span>
              {activePreset && (
                <span className="preview__status-chip">
                  {activePreset.label}
                </span>
              )}
            </div>
            <div className="preview__status-text">{status}</div>
          </div>

          <div className="preview__meta-grid">
            <div className={cx(mutedPanelClass, 'preview__meta')}>
              <div className="preview__label">History</div>
              <strong className="preview__meta-value">{historyDepth}</strong>
            </div>
            <div className={cx(mutedPanelClass, 'preview__meta')}>
              <div className="preview__label">Mode</div>
              <strong className="preview__meta-text">{isReady ? 'Ready to edit' : 'Loading source'}</strong>
            </div>
          </div>
        </div>
      </div>

      <img
        alt={sourceLabel}
        className="is-hidden"
        onLoad={syncCanvasWithSource}
        ref={imageRef}
        src={sourceImageUrl}
      />
    </section>
  );
}
