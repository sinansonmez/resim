import React from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';

const Component = () => {
    return <div onClick={wasm.greet} >Hello from React!</div>;
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component/>);