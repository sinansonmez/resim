import { useEffect, useRef, useState } from 'react';

import sampleImage from '../asset/bosphorus.jpg';
import { createDefaultControls, createTransformCatalog, formatControlValue } from '../demo-utils.mjs';
import { presetCatalog } from '../presets.mjs';

let wasmApiPromise;

function loadWasmApi() {
  if (!wasmApiPromise) {
    wasmApiPromise = import('../../rust/pkg/resim.js').then(async (module) => {
      await module.default();
      return module;
    });
  }

  return wasmApiPromise;
}

export function useResimEditor() {
  const canvasRef = useRef(null);
  const originalCanvasRef = useRef(null);
  const imageRef = useRef(null);
  const fileInputRef = useRef(null);
  const originalSnapshotRef = useRef(null);
  const wasmApiRef = useRef(null);
  const uploadedImageUrlRef = useRef(null);
  const historyRef = useRef([]);

  const [transformCatalog, setTransformCatalog] = useState({});
  const [selectedTransform, setSelectedTransform] = useState(null);
  const [selectedPreset, setSelectedPreset] = useState(presetCatalog[0]?.id ?? null);
  const [controlValues, setControlValues] = useState({});
  const [sourceImageUrl, setSourceImageUrl] = useState(sampleImage);
  const [sourceLabel, setSourceLabel] = useState('Sample image');
  const [resizeWidth, setResizeWidth] = useState('');
  const [resizeHeight, setResizeHeight] = useState('');
  const [isComparisonMode, setIsComparisonMode] = useState(false);
  const [isFineTuneOpen, setIsFineTuneOpen] = useState(false);
  const [historyDepth, setHistoryDepth] = useState(0);
  const [status, setStatus] = useState('Loading WebAssembly module');
  const [isReady, setIsReady] = useState(false);

  const transformEntries = Object.entries(transformCatalog);
  const fullPresets = presetCatalog.filter((preset) => !preset.quick);
  const quickPresets = presetCatalog.filter((preset) => preset.quick);
  const selectedConfig = selectedTransform ? transformCatalog[selectedTransform] : null;
  const selectedControl = selectedConfig?.control ?? null;
  const selectedControlValue = selectedControl ? controlValues[selectedTransform] : null;
  const activePreset =
    presetCatalog.find((preset) => preset.id === selectedPreset) ?? presetCatalog[0] ?? null;

  const createCanvasSnapshot = (canvas, ctx, wasmApi) => ({
    imageData: wasmApi.readCanvasImageData(canvas, ctx),
    width: canvas.width,
    height: canvas.height,
  });

  const writeSnapshotToCanvas = (canvas, ctx, wasmApi, snapshot) => {
    canvas.width = snapshot.width;
    canvas.height = snapshot.height;
    wasmApi.writeCanvasImageData(ctx, snapshot.imageData);
  };

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
      if (originalCtx) {
        originalCanvas.width = width;
        originalCanvas.height = height;
        originalCtx.clearRect(0, 0, width, height);
        originalCtx.drawImage(image, 0, 0, width, height);
      }
    }

    try {
      const originalSnapshot = createCanvasSnapshot(canvas, ctx, wasmApi);
      originalSnapshotRef.current = originalSnapshot;
      historyRef.current = [];
      setHistoryDepth(0);
      setResizeWidth(String(width));
      setResizeHeight(String(height));
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
      const currentSnapshot = createCanvasSnapshot(canvas, ctx, wasmApi);
      historyRef.current = [...historyRef.current, currentSnapshot];
      setHistoryDepth(historyRef.current.length);
      const transformedImageData = selectedControl
        ? wasmApi[selectedConfig.method](currentSnapshot.imageData, selectedControlValue)
        : wasmApi[selectedConfig.method](currentSnapshot.imageData);
      wasmApi.writeCanvasImageData(ctx, transformedImageData);

      const suffix = selectedControl
        ? ` (${formatControlValue(selectedControl, selectedControlValue)})`
        : '';
      setStatus(`${selectedConfig.label} applied${suffix}`);
    } catch (error) {
      setStatus(`Transform failed: ${String(error)}`);
    }
  };

  const selectAndApplyPreset = (presetId) => {
    setSelectedPreset(presetId);

    const preset = presetCatalog.find((entry) => entry.id === presetId);
    if (!preset) {
      return;
    }

    const canvas = canvasRef.current;
    const wasmApi = wasmApiRef.current;
    const originalSnapshot = originalSnapshotRef.current;

    if (!canvas || !wasmApi || !isReady || !originalSnapshot) {
      setStatus(`${preset.label} selected`);
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      const currentSnapshot = createCanvasSnapshot(canvas, ctx, wasmApi);
      historyRef.current = [...historyRef.current, currentSnapshot];
      setHistoryDepth(historyRef.current.length);
      writeSnapshotToCanvas(canvas, ctx, wasmApi, originalSnapshot);
      const baseSnapshot = createCanvasSnapshot(canvas, ctx, wasmApi);
      const nextImageData = wasmApi.applyPresetImageData(baseSnapshot.imageData, preset.id);
      writeSnapshotToCanvas(canvas, ctx, wasmApi, {
        imageData: nextImageData,
        width: baseSnapshot.width,
        height: baseSnapshot.height,
      });
      setResizeWidth(String(baseSnapshot.width));
      setResizeHeight(String(baseSnapshot.height));
      setStatus(`${preset.label} preset applied`);
    } catch (error) {
      setStatus(`Preset failed: ${String(error)}`);
    }
  };

  const resetPreview = () => {
    const canvas = canvasRef.current;
    const originalSnapshot = originalSnapshotRef.current;
    const wasmApi = wasmApiRef.current;

    if (!canvas || !originalSnapshot || !wasmApi) {
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      writeSnapshotToCanvas(canvas, ctx, wasmApi, originalSnapshot);
      historyRef.current = [];
      setHistoryDepth(0);
      setResizeWidth(String(originalSnapshot.width));
      setResizeHeight(String(originalSnapshot.height));
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

    const previousSnapshot = historyRef.current[historyRef.current.length - 1];
    historyRef.current = historyRef.current.slice(0, -1);
    setHistoryDepth(historyRef.current.length);

    try {
      writeSnapshotToCanvas(canvas, ctx, wasmApi, previousSnapshot);
      setResizeWidth(String(previousSnapshot.width));
      setResizeHeight(String(previousSnapshot.height));
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

  const resizePreview = () => {
    const canvas = canvasRef.current;
    const wasmApi = wasmApiRef.current;
    const nextWidth = Number(resizeWidth);
    const nextHeight = Number(resizeHeight);

    if (!canvas || !wasmApi || !isReady) {
      return;
    }

    if (
      !Number.isInteger(nextWidth) ||
      !Number.isInteger(nextHeight) ||
      nextWidth < 1 ||
      nextHeight < 1
    ) {
      setStatus('Resize requires integer width and height greater than zero');
      return;
    }

    const ctx = canvas.getContext('2d');
    if (!ctx) {
      setStatus('Canvas is unavailable in this browser');
      return;
    }

    try {
      const currentSnapshot = createCanvasSnapshot(canvas, ctx, wasmApi);
      historyRef.current = [...historyRef.current, currentSnapshot];
      setHistoryDepth(historyRef.current.length);
      const resizedImageData = wasmApi.resizeImageData(
        currentSnapshot.imageData,
        nextWidth,
        nextHeight
      );

      writeSnapshotToCanvas(canvas, ctx, wasmApi, {
        imageData: resizedImageData,
        width: nextWidth,
        height: nextHeight,
      });
      setStatus(`Resized preview to ${nextWidth}x${nextHeight}`);
    } catch (error) {
      setStatus(`Resize failed: ${String(error)}`);
    }
  };

  const restoreSampleImage = () => {
    if (uploadedImageUrlRef.current) {
      URL.revokeObjectURL(uploadedImageUrlRef.current);
      uploadedImageUrlRef.current = null;
    }

    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }

    originalSnapshotRef.current = null;
    historyRef.current = [];
    setHistoryDepth(0);
    setIsReady(false);
    setSourceLabel('Sample image');
    setSourceImageUrl(sampleImage);
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
    originalSnapshotRef.current = null;
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

  return {
    activePreset,
    applySelectedTransform,
    canvasRef,
    downloadPreview,
    fileInputRef,
    fullPresets,
    handleFileSelection,
    historyDepth,
    imageRef,
    isComparisonMode,
    isFineTuneOpen,
    isReady,
    originalCanvasRef,
    quickPresets,
    resizeHeight,
    resizePreview,
    resizeWidth,
    resetPreview,
    restoreSampleImage,
    selectAndApplyPreset,
    selectedConfig,
    selectedControl,
    selectedControlValue,
    selectedPreset,
    selectedTransform,
    setIsComparisonMode,
    setIsFineTuneOpen,
    setResizeHeight,
    setResizeWidth,
    setSelectedTransform,
    sourceImageUrl,
    sourceLabel,
    status,
    syncCanvasWithSource,
    transformEntries,
    undoLastStep,
    updateControlValue,
  };
}
