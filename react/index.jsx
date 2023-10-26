import React, { useEffect } from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpg'

const Component = () => {
    const onClickHandler = (imageUrl) => {
        console.time('start')
        wasm.greet(imageUrl)
        console.timeEnd('start')
    }
    return (
        <>
            <div style={{ display: 'flex' }} >
                <button onClick={() => onClickHandler(cat)} >Hello from React!</button>
            </div>
            <div>
                <img src={cat} width={250} />
            </div>
        </>
    );
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component />);