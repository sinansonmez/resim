import React from 'react';
import { createRoot } from 'react-dom/client';

import { App } from './app/App.jsx';
import './styles.css';

const rootElement = document.getElementById('app');

if (!rootElement) {
  throw new Error('Unable to find #app root element');
}

createRoot(rootElement).render(<App />);
