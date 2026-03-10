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

const transformMetadata = {
  grayscale: {
    label: 'Grayscale',
    description: 'Average RGB channels for a classic monochrome output.',
    method: 'grayscaleImageData',
  },
  invert: {
    label: 'Invert',
    description: 'Flip every color channel while preserving transparency.',
    method: 'invertImageData',
  },
  blur: {
    label: 'Blur',
    description: 'Apply a simple 3x3 box blur to soften the image.',
    method: 'blurImageData',
  },
};

const shellStyle = {
  minHeight: '100vh',
  margin: 0,
  fontFamily: '"Space Grotesk", "Segoe UI", sans-serif',
  color: '#132238',
  background:
    'radial-gradient(circle at top left, #f7c873 0%, #f4e7cc 30%, #cfe4db 68%, #9ec5c4 100%)',
};

const pageStyle = {
  maxWidth: '1200px',
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
  maxWidth: '720px',
  margin: 0,
  fontSize: '1.05rem',
  lineHeight: 1.7,
};

const layoutStyle = {
  display: 'grid',
  gap: '24px',
  gridTemplateColumns: 'minmax(280px, 340px) minmax(0, 1fr)',
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

  @media (max-width: 860px) {
    .resim-layout {
      grid-template-columns: 1fr !important;
    }
  }
`;

function Component() {
  const canvasRef = useRef(null);
  const imageRef = useRef(null);
  const originalImageDataRef = useRef(null);
  const wasmApiRef = useRef(null);

  const [selectedTransform, setSelectedTransform] = useState('grayscale');
  const [status, setStatus] = useState('Loading WebAssembly module');
  const [isReady, setIsReady] = useState(false);

  const syncCanvasWithSource = () => {
    const image = imageRef.current;
    const canvas = canvasRef.current;
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

    try {
      const originalImageData = wasmApi.readCanvasImageData(canvas, ctx);
      originalImageDataRef.current = originalImageData;
      setIsReady(true);
      setStatus('Sample image loaded');
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

  const applySelectedTransform = () => {
    const canvas = canvasRef.current;
    const wasmApi = wasmApiRef.current;

    if (!canvas || !isReady || !wasmApi) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      const currentImageData = wasmApi.readCanvasImageData(canvas, ctx);
      const transformedImageData =
        wasmApi[transformMetadata[selectedTransform].method](currentImageData);
      wasmApi.writeCanvasImageData(ctx, transformedImageData);
      setStatus(`${transformMetadata[selectedTransform].label} applied`);
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
      setStatus('Preview reset to the original image');
    } catch (error) {
      setStatus(`Reset failed: ${String(error)}`);
    }
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
            The library stays focused on a tight core API while this page demonstrates
            how the generated wasm package can drive a real canvas workflow.
          </p>
        </section>

        <section className="resim-layout" style={layoutStyle}>
          <aside style={panelStyle}>
            <div style={cardStyle}>
              <h2 style={{ margin: 0, fontSize: '1.4rem' }}>Transform controls</h2>
              <p style={{ margin: 0, lineHeight: 1.6 }}>
                Choose a transform, apply it to the current preview, or reset the canvas
                back to the original sample image.
              </p>
            </div>

            <div style={{ display: 'grid', gap: '12px', marginBottom: '18px' }}>
              {Object.entries(transformMetadata).map(([key, transform]) => {
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

            <div style={{ display: 'flex', gap: '12px', flexWrap: 'wrap', marginBottom: '18px' }}>
              <button
                onClick={applySelectedTransform}
                style={{
                  ...primaryButtonStyle,
                  background: '#ea6a47',
                  color: '#fff8f0',
                  flex: '1 1 180px',
                  opacity: isReady ? 1 : 0.6,
                }}
                type="button"
                disabled={!isReady}
              >
                Apply {transformMetadata[selectedTransform].label}
              </button>
              <button
                onClick={resetPreview}
                style={{
                  ...primaryButtonStyle,
                  background: 'rgba(19, 34, 56, 0.08)',
                  color: '#132238',
                  flex: '1 1 160px',
                  opacity: isReady ? 1 : 0.6,
                }}
                type="button"
                disabled={!isReady}
              >
                Reset Preview
              </button>
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
                    The sample image is drawn to a canvas, then transformed in-place with
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
                  {transformMetadata[selectedTransform].label}
                </code>
              </div>

              <canvas ref={canvasRef} style={canvasStyle} />
              <img
                alt="Sample source"
                onLoad={syncCanvasWithSource}
                ref={imageRef}
                src={cat}
                style={{ display: 'none' }}
              />
            </section>

            <section style={panelStyle}>
              <h2 style={{ marginTop: 0, fontSize: '1.3rem' }}>Library shape</h2>
              <p style={{ marginTop: 0, lineHeight: 1.7 }}>
                The Rust core now owns the pixel math, while wasm bindings expose a small
                browser API around <code>ImageData</code> and canvas read/write helpers.
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
  grayscaleImageData,
  writeCanvasImageData,
} from "resim";

await init();
const imageData = readCanvasImageData(canvas, ctx);
const next = grayscaleImageData(imageData);
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
