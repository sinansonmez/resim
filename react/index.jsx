import React, { useEffect } from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpg'
import dice from './asset/dice.png'

const Component = () => {

    async function loadImageAsUint8Array(imagePath) {
        try {
            const response = await fetch(imagePath);
            if (!response.ok) {
                throw new Error('Network response was not ok');
            }
            const arrayBuffer = await response.arrayBuffer();
            // Create a Uint8Array from the array buffer
            const uint8Array = new Uint8Array(arrayBuffer);
            return uint8Array;
        } catch (error) {
            console.error('Error:', error);
        }
    }

    const onClickHandler = async (imageUrl) => {
        console.time('start')
        const uint8Array = await loadImageAsUint8Array(imageUrl)
        if (uint8Array) {
            console.log('Uint8Array:', uint8Array);
            const image = wasm.greet(uint8Array)
            // Create a Blob from the Uint8Array
            const blob = new Blob([image], { type: 'image/png' });

            // Create a Data URL from the Blob
            const url = URL.createObjectURL(blob);
            document.getElementById('new').src = url;
            console.log("new image: ", image);
        } else {
            console.err('Error loading the image.');
        }
        console.timeEnd('start')
    }
    return (
        <>
            <div style={{ display: 'flex' }} >
                <button onClick={() => onClickHandler(dice)} >Hello from React!</button>
            </div>
            <div>
                <img src={dice} width={250} />
                <img id='new' src={cat} width={250} />
            </div>
        </>
    );
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component />);