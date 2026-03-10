import React, { useEffect, useRef, useState } from 'react';
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpeg';

let wasmApiPromise;

function loadWasmApi() {
  if (!wasmApiPromise) {
    wasmApiPromise = import('../rust/pkg/resim.js').then(async (module) => {
      await module.default();
      return module;
    });
  }

  return wasmApiPromise;
}

function createTransformCatalog(entries) {
  return Object.fromEntries(entries.map((entry) => [entry.id, entry]));
}

function createDefaultControls(entries) {
  return Object.fromEntries(
    entries
      .filter((transform) => transform.control)
      .map((transform) => [transform.id, transform.control.defaultValue])
  );
}

function formatControlValue(control, value) {
  switch (control.display) {
    case 'signed':
      return `${value > 0 ? '+' : ''}${value}`;
    case 'signedPercent':
      return `${value > 0 ? '+' : ''}${value}%`;
    default:
      return `${value}`;
  }
}

const shellStyle = {
  minHeight: '100vh',
  margin: 0,
  fontFamily: '"Space Grotesk", "Segoe UI", sans-serif',
  color: '#132238',
  background:
    'radial-gradient(circle at top left, #f7c873 0%, #f4e7cc 30%, #cfe4db 68%, #9ec5c4 100%)',
};

const pageStyle = {
  maxWidth: '1220px',
  margin: '0 auto',
  padding: '48px 24px 64px',
};

const heroStyle = {
  display: 'grid',
  gap: '20px',
  marginBottom: '28px',
};

const badgeStyle = {
  width: 'fit-content',
  padding: '8px 12px',
  borderRadius: '999px',
  background: 'rgba(19, 34, 56, 0.08)',
  fontSize: '12px',
  letterSpacing: '0.08em',
  textTransform: 'uppercase',
};

const titleStyle = {
  margin: 0,
  fontSize: 'clamp(2.8rem, 7vw, 5.8rem)',
  lineHeight: 0.95,
  letterSpacing: '-0.06em',
};

const subtitleStyle = {
  maxWidth: '760px',
  margin: 0,
  fontSize: '1.05rem',
  lineHeight: 1.7,
};

const layoutStyle = {
  display: 'grid',
  gap: '24px',
  gridTemplateColumns: 'minmax(300px, 360px) minmax(0, 1fr)',
  alignItems: 'start',
};

const panelStyle = {
  border: '1px solid rgba(19, 34, 56, 0.08)',
  borderRadius: '28px',
  padding: '24px',
  background: 'rgba(255, 252, 246, 0.72)',
  backdropFilter: 'blur(18px)',
  boxShadow: '0 24px 64px rgba(19, 34, 56, 0.08)',
};

const buttonBaseStyle = {
  width: '100%',
  border: 'none',
  borderRadius: '18px',
  padding: '14px 16px',
  textAlign: 'left',
  cursor: 'pointer',
  transition: 'transform 160ms ease, box-shadow 160ms ease, background 160ms ease',
};

const primaryButtonStyle = {
  border: 'none',
  borderRadius: '18px',
  padding: '14px 18px',
  fontSize: '0.95rem',
  fontWeight: 600,
  cursor: 'pointer',
};

const canvasFrameStyle = {
  ...panelStyle,
  overflow: 'hidden',
  padding: '20px',
};

const canvasStyle = {
  display: 'block',
  width: '100%',
  borderRadius: '20px',
  background: '#fff',
  boxShadow: 'inset 0 0 0 1px rgba(19, 34, 56, 0.06)',
};

const cardStyle = {
  display: 'grid',
  gap: '10px',
  marginBottom: '12px',
};

const surfaceStyles = `
  * {
    box-sizing: border-box;
  }

  body {
    margin: 0;
  }

  button:hover {
    transform: translateY(-1px);
  }

  input,
  input[type="range"] {
    width: 100%;
  }

  input[type="range"] {
    accent-color: #ea6a47;
  }

  input[type="checkbox"] {
    width: 18px;
    height: 18px;
    accent-color: #132238;
  }

  @media (max-width: 860px) {
    .resim-layout {
      grid-template-columns: 1fr !important;
    }

    .resim-action-grid {
      grid-template-columns: 1fr !important;
    }

    .resim-compare-grid {
      grid-template-columns: 1fr !important;
    }
  }
`;

function Component() {
  const canvasRef = useRef(null);
  const originalCanvasRef = useRef(null);
  const imageRef = useRef(null);
  const fileInputRef = useRef(null);
  const originalImageDataRef = useRef(null);
  const wasmApiRef = useRef(null);
  const uploadedImageUrlRef = useRef(null);
  const historyRef = useRef([]);

  const [transformCatalog, setTransformCatalog] = useState({});
  const [selectedTransform, setSelectedTransform] = useState(null);
  const [controlValues, setControlValues] = useState({});
  const [sourceImageUrl, setSourceImageUrl] = useState(cat);
  const [sourceLabel, setSourceLabel] = useState('Sample image');
  const [isComparisonMode, setIsComparisonMode] = useState(false);
  const [historyDepth, setHistoryDepth] = useState(0);
  const [status, setStatus] = useState('Loading WebAssembly module');
  const [isReady, setIsReady] = useState(false);

  const transformEntries = Object.entries(transformCatalog);
  const selectedConfig = selectedTransform ? transformCatalog[selectedTransform] : null;
  const selectedControl = selectedConfig?.control ?? null;
  const selectedControlValue = selectedControl ? controlValues[selectedTransform] : null;

  const syncCanvasWithSource = () => {
    const image = imageRef.current;
    const canvas = canvasRef.current;
    const originalCanvas = originalCanvasRef.current;
    const wasmApi = wasmApiRef.current;

    if (!image || !canvas || !wasmApi) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    const width = image.naturalWidth || image.width;
    const height = image.naturalHeight || image.height;

    if (!width || !height) {
      return;
    }

    canvas.width = width;
    canvas.height = height;
    ctx.clearRect(0, 0, width, height);
    ctx.drawImage(image, 0, 0, width, height);

    if (originalCanvas) {
      const originalCtx = originalCanvas.getContext('2d');
      if (!originalCtx) {
        setStatus('Comparison canvas is unavailable in this browser');
        return;
      }

      originalCanvas.width = width;
      originalCanvas.height = height;
      originalCtx.clearRect(0, 0, width, height);
      originalCtx.drawImage(image, 0, 0, width, height);
    }

    try {
      const originalImageData = wasmApi.readCanvasImageData(canvas, ctx);
      originalImageDataRef.current = originalImageData;
      historyRef.current = [];
      setHistoryDepth(0);
      setIsReady(true);
      setStatus(`${sourceLabel} loaded`);
    } catch (error) {
      setStatus(`Unable to read the canvas: ${String(error)}`);
    }
  };

  useEffect(() => {
    let cancelled = false;

    loadWasmApi()
      .then((module) => {
        if (cancelled) {
          return;
        }

        wasmApiRef.current = module;
        const catalogEntries = module.getTransformCatalog ? module.getTransformCatalog() : [];
        const nextCatalog = createTransformCatalog(catalogEntries);
        const nextControlValues = createDefaultControls(catalogEntries);
        const firstTransform = catalogEntries[0]?.id ?? null;

        setTransformCatalog(nextCatalog);
        setControlValues(nextControlValues);
        setSelectedTransform(firstTransform);
        setStatus('WebAssembly module ready');

        const image = imageRef.current;
        if (image && image.complete) {
          syncCanvasWithSource();
        }
      })
      .catch((error) => {
        if (!cancelled) {
          setStatus(`Unable to load WebAssembly: ${String(error)}`);
        }
      });

    return () => {
      cancelled = true;
    };
  }, []);

  useEffect(() => () => {
    if (uploadedImageUrlRef.current) {
      URL.revokeObjectURL(uploadedImageUrlRef.current);
    }
  }, []);

  useEffect(() => {
    if (!isComparisonMode) {
      return;
    }

    syncCanvasWithSource();
  }, [isComparisonMode, sourceImageUrl]);

  const applySelectedTransform = () => {
    const canvas = canvasRef.current;
    const wasmApi = wasmApiRef.current;

    if (!canvas || !isReady || !wasmApi || !selectedConfig) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      const currentImageData = wasmApi.readCanvasImageData(canvas, ctx);
      historyRef.current = [...historyRef.current, currentImageData];
      setHistoryDepth(historyRef.current.length);
      const transformedImageData = selectedControl
        ? wasmApi[selectedConfig.method](currentImageData, selectedControlValue)
        : wasmApi[selectedConfig.method](currentImageData);
      wasmApi.writeCanvasImageData(ctx, transformedImageData);

      const suffix = selectedControl
        ? ` (${formatControlValue(selectedControl, selectedControlValue)})`
        : '';
      setStatus(`${selectedConfig.label} applied${suffix}`);
    } catch (error) {
      setStatus(`Transform failed: ${String(error)}`);
    }
  };

  const resetPreview = () => {
    const canvas = canvasRef.current;
    const originalImageData = originalImageDataRef.current;
    const wasmApi = wasmApiRef.current;

    if (!canvas || !originalImageData || !wasmApi) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      wasmApi.writeCanvasImageData(ctx, originalImageData);
      historyRef.current = [];
      setHistoryDepth(0);
      setStatus('Preview reset to the original image');
    } catch (error) {
      setStatus(`Reset failed: ${String(error)}`);
    }
  };

  const undoLastStep = () => {
    const canvas = canvasRef.current;
    const wasmApi = wasmApiRef.current;

    if (!canvas || !wasmApi || historyRef.current.length === 0) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    const previousImageData = historyRef.current[historyRef.current.length - 1];
    historyRef.current = historyRef.current.slice(0, -1);
    setHistoryDepth(historyRef.current.length);

    try {
      wasmApi.writeCanvasImageData(ctx, previousImageData);
      setStatus('Reverted the last transform');
    } catch (error) {
      setStatus(`Undo failed: ${String(error)}`);
    }
  };

  const updateControlValue = (event) => {
    const nextValue = Number(event.target.value);
    setControlValues((current) => ({
      ...current,
      [selectedTransform]: nextValue,
    }));
  };

  const restoreSampleImage = () => {
    if (uploadedImageUrlRef.current) {
      URL.revokeObjectURL(uploadedImageUrlRef.current);
      uploadedImageUrlRef.current = null;
    }

    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }

    originalImageDataRef.current = null;
    historyRef.current = [];
    setHistoryDepth(0);
    setIsReady(false);
    setSourceLabel('Sample image');
    setSourceImageUrl(cat);
    setStatus('Loading sample image');
  };

  const handleFileSelection = (event) => {
    const [file] = event.target.files || [];
    if (!file) {
      return;
    }

    if (!file.type.startsWith('image/')) {
      setStatus('Please choose an image file');
      event.target.value = '';
      return;
    }

    if (uploadedImageUrlRef.current) {
      URL.revokeObjectURL(uploadedImageUrlRef.current);
    }

    const nextUrl = URL.createObjectURL(file);
    uploadedImageUrlRef.current = nextUrl;
    originalImageDataRef.current = null;
    historyRef.current = [];
    setHistoryDepth(0);
    setIsReady(false);
    setSourceLabel(file.name);
    setSourceImageUrl(nextUrl);
    setStatus(`Loading ${file.name}`);
  };

  const downloadPreview = () => {
    const canvas = canvasRef.current;

    if (!canvas || !isReady) {
      return;
    }

    canvas.toBlob((blob) => {
      if (!blob) {
        setStatus('Download failed: unable to generate PNG');
        return;
      }

      const downloadUrl = URL.createObjectURL(blob);
      const link = document.createElement('a');
      link.href = downloadUrl;
      link.download = 'resim-output.png';
      link.click();
      URL.revokeObjectURL(downloadUrl);
      setStatus('Downloaded processed image as PNG');
    }, 'image/png');
  };

  return (
    <main style={shellStyle}>
      <style>{surfaceStyles}</style>
      <div style={pageStyle}>
        <section style={heroStyle}>
          <span style={badgeStyle}>Rust + WebAssembly image processing</span>
          <h1 style={titleStyle}>Resim</h1>
          <p style={subtitleStyle}>
            A small open-source showcase for browser image transforms powered by Rust.
            The library now includes practical parameterized filters while keeping the
            JavaScript surface centered on <code>ImageData</code>.
          </p>
        </section>

        <section className="resim-layout" style={layoutStyle}>
          <aside style={panelStyle}>
            <div style={cardStyle}>
              <h2 style={{ margin: 0, fontSize: '1.4rem' }}>Transform controls</h2>
              <p style={{ margin: 0, lineHeight: 1.6 }}>
                Choose a transform, tune any available parameter, apply it to the current
                preview, upload your own image, download the current result, or reset the
                canvas back to the original source. Comparison mode and undo make it easier
                to inspect incremental changes. The control list is now driven by the wasm
                transform catalog instead of a separate demo-only map.
              </p>
            </div>

            <div
              style={{
                display: 'grid',
                gap: '12px',
                marginBottom: '18px',
                padding: '16px',
                borderRadius: '20px',
                background: 'rgba(19, 34, 56, 0.05)',
              }}
            >
              <div>
                <strong style={{ display: 'block', marginBottom: '6px' }}>Image source</strong>
                <span style={{ lineHeight: 1.5 }}>{sourceLabel}</span>
              </div>
              <input
                accept="image/*"
                onChange={handleFileSelection}
                ref={fileInputRef}
                style={{ display: 'none' }}
                type="file"
              />
              <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap' }}>
                <button
                  onClick={() => fileInputRef.current?.click()}
                  style={{
                    ...primaryButtonStyle,
                    background: '#132238',
                    color: '#f8f2e8',
                    flex: '1 1 180px',
                  }}
                  type="button"
                >
                  Upload Image
                </button>
                <button
                  onClick={restoreSampleImage}
                  style={{
                    ...primaryButtonStyle,
                    background: 'rgba(19, 34, 56, 0.08)',
                    color: '#132238',
                    flex: '1 1 160px',
                  }}
                  type="button"
                >
                  Use Sample
                </button>
              </div>
            </div>

            <div style={{ display: 'grid', gap: '12px', marginBottom: '18px' }}>
              {transformEntries.map(([key, transform]) => {
                const isSelected = key === selectedTransform;

                return (
                  <button
                    key={key}
                    onClick={() => setSelectedTransform(key)}
                    style={{
                      ...buttonBaseStyle,
                      background: isSelected ? '#132238' : 'rgba(19, 34, 56, 0.05)',
                      color: isSelected ? '#f8f2e8' : '#132238',
                      boxShadow: isSelected
                        ? '0 16px 28px rgba(19, 34, 56, 0.2)'
                        : 'none',
                      transform: isSelected ? 'translateY(-1px)' : 'translateY(0)',
                    }}
                    type="button"
                  >
                    <strong style={{ display: 'block', marginBottom: '6px' }}>
                      {transform.label}
                    </strong>
                    <span style={{ display: 'block', lineHeight: 1.5 }}>
                      {transform.description}
                    </span>
                  </button>
                );
              })}
            </div>

            {selectedControl && (
              <div
                style={{
                  marginBottom: '18px',
                  padding: '16px',
                  borderRadius: '20px',
                  background: 'rgba(19, 34, 56, 0.05)',
                }}
              >
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    gap: '12px',
                    marginBottom: '10px',
                    alignItems: 'center',
                  }}
                >
                  <strong>{selectedControl.label}</strong>
                  <code>{formatControlValue(selectedControl, selectedControlValue)}</code>
                </div>
                <input
                  max={selectedControl.max}
                  min={selectedControl.min}
                  onChange={updateControlValue}
                  step={selectedControl.step}
                  type="range"
                  value={selectedControlValue}
                />
                <div
                  style={{
                    display: 'flex',
                    justifyContent: 'space-between',
                    marginTop: '8px',
                    fontSize: '0.82rem',
                    opacity: 0.72,
                  }}
                >
                  <span>{selectedControl.min}</span>
                  <span>{selectedControl.max}</span>
                </div>
              </div>
            )}

            <div
              className="resim-action-grid"
              style={{
                display: 'grid',
                gap: '12px',
                gridTemplateColumns: 'repeat(2, minmax(0, 1fr))',
                marginBottom: '18px',
              }}
            >
              <button
                onClick={applySelectedTransform}
                style={{
                  ...primaryButtonStyle,
                  background: '#ea6a47',
                  color: '#fff8f0',
                  opacity: isReady ? 1 : 0.6,
                }}
                type="button"
                disabled={!isReady || !selectedConfig}
              >
                Apply {selectedConfig ? selectedConfig.label : 'Transform'}
              </button>
              <button
                onClick={resetPreview}
                style={{
                  ...primaryButtonStyle,
                  background: 'rgba(19, 34, 56, 0.08)',
                  color: '#132238',
                  opacity: isReady ? 1 : 0.6,
                }}
                type="button"
                disabled={!isReady}
              >
                Reset Preview
              </button>
              <button
                onClick={undoLastStep}
                style={{
                  ...primaryButtonStyle,
                  background: 'rgba(19, 34, 56, 0.12)',
                  color: '#132238',
                  opacity: historyDepth > 0 ? 1 : 0.6,
                }}
                type="button"
                disabled={historyDepth === 0}
              >
                Undo Last Step
              </button>
              <button
                onClick={downloadPreview}
                style={{
                  ...primaryButtonStyle,
                  background: '#2d8f74',
                  color: '#f5fffb',
                  opacity: isReady ? 1 : 0.6,
                }}
                type="button"
                disabled={!isReady}
              >
                Download PNG
              </button>
            </div>

            <div
              style={{
                display: 'grid',
                gap: '10px',
                marginBottom: '18px',
                padding: '16px',
                borderRadius: '20px',
                background: 'rgba(19, 34, 56, 0.05)',
              }}
            >
              <label
                style={{
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'space-between',
                  gap: '16px',
                  cursor: 'pointer',
                }}
              >
                <span>
                  <strong style={{ display: 'block', marginBottom: '4px' }}>Comparison mode</strong>
                  <span style={{ lineHeight: 1.5 }}>
                    Show the untouched source beside the processed canvas.
                  </span>
                </span>
                <input
                  checked={isComparisonMode}
                  onChange={(event) => setIsComparisonMode(event.target.checked)}
                  type="checkbox"
                />
              </label>
              <span style={{ fontSize: '0.82rem', opacity: 0.72 }}>
                Undo depth: {historyDepth}
              </span>
            </div>

            <div
              style={{
                borderRadius: '20px',
                padding: '16px',
                background: 'rgba(19, 34, 56, 0.06)',
                lineHeight: 1.6,
              }}
            >
              <strong style={{ display: 'block', marginBottom: '6px' }}>Status</strong>
              <span>{status}</span>
            </div>
          </aside>

          <div style={{ display: 'grid', gap: '18px' }}>
            <section style={canvasFrameStyle}>
              <div
                style={{
                  display: 'flex',
                  justifyContent: 'space-between',
                  gap: '12px',
                  alignItems: 'center',
                  flexWrap: 'wrap',
                  marginBottom: '16px',
                }}
              >
                <div>
                  <h2 style={{ margin: '0 0 6px', fontSize: '1.4rem' }}>Preview</h2>
                  <p style={{ margin: 0, lineHeight: 1.6 }}>
                    The active source is drawn to a canvas, then transformed in-place with
                    the wasm export selected on the left.
                  </p>
                </div>
                <code
                  style={{
                    padding: '8px 12px',
                    borderRadius: '999px',
                    background: 'rgba(19, 34, 56, 0.06)',
                    fontSize: '0.82rem',
                  }}
                >
                  {selectedControl
                    ? `${selectedConfig.label} ${formatControlValue(selectedControl, selectedControlValue)}`
                    : selectedConfig?.label ?? 'Loading'}
                </code>
              </div>

              <div
                className={isComparisonMode ? 'resim-compare-grid' : undefined}
                style={{
                  display: 'grid',
                  gap: '16px',
                  gridTemplateColumns: isComparisonMode ? 'repeat(2, minmax(0, 1fr))' : '1fr',
                }}
              >
                {isComparisonMode && (
                  <div style={{ display: 'grid', gap: '10px' }}>
                    <strong style={{ fontSize: '0.92rem' }}>Original</strong>
                    <canvas ref={originalCanvasRef} style={canvasStyle} />
                  </div>
                )}
                <div style={{ display: 'grid', gap: '10px' }}>
                  <strong style={{ fontSize: '0.92rem' }}>
                    {isComparisonMode ? 'Processed' : 'Canvas'}
                  </strong>
                  <canvas ref={canvasRef} style={canvasStyle} />
                </div>
              </div>
              <img
                alt={sourceLabel}
                onLoad={syncCanvasWithSource}
                ref={imageRef}
                src={sourceImageUrl}
                style={{ display: 'none' }}
              />
            </section>

            <section style={panelStyle}>
              <h2 style={{ marginTop: 0, fontSize: '1.3rem' }}>Library shape</h2>
              <p style={{ marginTop: 0, lineHeight: 1.7 }}>
                The Rust core owns pixel math, while wasm bindings expose additive browser
                APIs around <code>ImageData</code>. Parameterized transforms stay explicit
                and predictable at the callsite.
              </p>
              <pre
                style={{
                  margin: 0,
                  padding: '18px',
                  borderRadius: '20px',
                  overflowX: 'auto',
                  background: '#132238',
                  color: '#f5ede1',
                }}
              >
{`import {
  default as init,
  readCanvasImageData,
  brightnessImageData,
  writeCanvasImageData,
} from "resim";

await init();
const imageData = readCanvasImageData(canvas, ctx);
const next = brightnessImageData(imageData, 30);
writeCanvasImageData(ctx, next);`}
              </pre>
            </section>
          </div>
        </section>
      </div>
    </main>
  );
}

const root = createRoot(document.getElementById('app'));
root.render(<Component />);
