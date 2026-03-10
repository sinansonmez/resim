import React from 'react';

import { FineTunePanel } from '../components/FineTunePanel.jsx';
import { HeroSection } from '../components/HeroSection.jsx';
import { PresetLibrary } from '../components/PresetLibrary.jsx';
import { PreviewPanel } from '../components/PreviewPanel.jsx';
import { SourcePanel } from '../components/SourcePanel.jsx';
import { TechnicalPanel } from '../components/TechnicalPanel.jsx';
import { useResimEditor } from './useResimEditor.js';

export function App() {
  const editor = useResimEditor();
  const presetCount = editor.quickPresets.length + editor.fullPresets.length;

  return (
    <main className="app-shell">
      <div className="app-shell__grid" />
      <div className="app-shell__glow app-shell__glow--blue" />
      <div className="app-shell__glow app-shell__glow--green" />
      <div className="app-shell__glow app-shell__glow--orange" />

      <div className="app-shell__content">
        <input
          accept="image/*"
          className="is-hidden"
          onChange={editor.handleFileSelection}
          ref={editor.fileInputRef}
          type="file"
        />

        <div className="topbar">
          <div className="topbar__badge">
            <span className="topbar__dot" />
            Resim browser demo
          </div>
          <div className="topbar__summary">
            Upload, preview, fine tune, export
          </div>
        </div>

        <HeroSection
          activePreset={editor.activePreset}
          historyDepth={editor.historyDepth}
          isReady={editor.isReady}
          onUploadClick={() => editor.fileInputRef.current?.click()}
          presetCount={presetCount}
          quickPresetCount={editor.quickPresets.length}
          sourceLabel={editor.sourceLabel}
        />

        <section className="layout-grid">
          <div className="workspace-column">
            <PreviewPanel
              activePreset={editor.activePreset}
              canvasRef={editor.canvasRef}
              downloadPreview={editor.downloadPreview}
              historyDepth={editor.historyDepth}
              imageRef={editor.imageRef}
              isComparisonMode={editor.isComparisonMode}
              isFineTuneOpen={editor.isFineTuneOpen}
              isReady={editor.isReady}
              originalCanvasRef={editor.originalCanvasRef}
              resetPreview={editor.resetPreview}
              setIsComparisonMode={editor.setIsComparisonMode}
              setIsFineTuneOpen={editor.setIsFineTuneOpen}
              sourceImageUrl={editor.sourceImageUrl}
              sourceLabel={editor.sourceLabel}
              status={editor.status}
              syncCanvasWithSource={editor.syncCanvasWithSource}
              undoLastStep={editor.undoLastStep}
            />
          </div>

          <aside className="sidebar-column">
            <SourcePanel
              isReady={editor.isReady}
              onUploadClick={() => editor.fileInputRef.current?.click()}
              restoreSampleImage={editor.restoreSampleImage}
              sourceLabel={editor.sourceLabel}
            />

            <FineTunePanel
              applySelectedTransform={editor.applySelectedTransform}
              historyDepth={editor.historyDepth}
              isFineTuneOpen={editor.isFineTuneOpen}
              isReady={editor.isReady}
              resizeHeight={editor.resizeHeight}
              resizePreview={editor.resizePreview}
              resizeWidth={editor.resizeWidth}
              selectedConfig={editor.selectedConfig}
              selectedControl={editor.selectedControl}
              selectedControlValue={editor.selectedControlValue}
              selectedTransform={editor.selectedTransform}
              setIsFineTuneOpen={editor.setIsFineTuneOpen}
              setResizeHeight={editor.setResizeHeight}
              setResizeWidth={editor.setResizeWidth}
              setSelectedTransform={editor.setSelectedTransform}
              sourceLabel={editor.sourceLabel}
              transformEntries={editor.transformEntries}
              updateControlValue={editor.updateControlValue}
            />

            <PresetLibrary
              activePreset={editor.activePreset}
              fullPresets={editor.fullPresets}
              quickPresets={editor.quickPresets}
              selectAndApplyPreset={editor.selectAndApplyPreset}
              selectedPreset={editor.selectedPreset}
            />

            <TechnicalPanel />
          </aside>
        </section>
      </div>
    </main>
  );
}
