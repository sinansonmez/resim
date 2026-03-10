import React from 'react';

import { cx, mutedPanelClass, panelClass, pillClass } from './ui.js';

export function TechnicalPanel() {
  return (
    <section className={cx(panelClass, 'technical-panel reveal reveal--delay-2')}>
      <div className="technical-panel__pills">
        <span className={pillClass}>Under the hood</span>
        <span className="pill pill--dark">Wasm</span>
      </div>
      <h3 className="section-title section-title--small">Rust + WebAssembly core</h3>
      <p className="section-copy">
        The editor stays <code>ImageData</code>-first and uses wasm bindings for the actual
        image work, while the demo acts as a curated product front-end instead of a raw tool
        dump.
      </p>
      <div className={cx(mutedPanelClass, 'technical-panel__code-shell')}>
        <pre className="technical-panel__code">
{`import {
  default as init,
  readImageDataFromCanvas,
  applyPresetImageData,
  writeImageDataToCanvas,
} from "resim";

await init();
const imageData = readImageDataFromCanvas(canvas, ctx);
const next = applyPresetImageData(imageData, "film-portrait");
writeImageDataToCanvas(ctx, next);`}
        </pre>
      </div>
    </section>
  );
}
