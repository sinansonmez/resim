import React, { useEffect } from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpg'
import dice from './asset/dice.png'

const Component = () => {

    function loadImageAsUint8Array(imagePath, callback) {
        const xhr = new XMLHttpRequest();
        xhr.open('GET', imagePath, true);
        xhr.responseType = 'arraybuffer';

        xhr.onload = function () {
            if (xhr.status === 200) {
                const arrayBuffer = xhr.response;
                const uint8Array = new Uint8Array(arrayBuffer);
                callback(uint8Array);
            } else {
                console.error('Failed to load the image.');
                callback(null);
            }
        };

        xhr.send();
    }

    const onClickHandler = (imageUrl) => {
        console.time('start')
        loadImageAsUint8Array(imageUrl, function (uint8Array) {
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
        });
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