import React, { useEffect } from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpg'
import dice from './asset/dice.png'

const Component = () => {

    // Function to convert an image to a Base64 encoded string
    function getBase64Image(imagePath, callback) {
        const reader = new FileReader();
      
        reader.onload = function () {
          if (typeof callback === 'function') {
            callback(reader.result);
          }
        };
      
        // Assuming imagePath is a valid local path to the image file
        fetch(imagePath)
          .then((response) => response.blob())
          .then((blob) => {
            reader.readAsDataURL(blob);
          })
          .catch((error) => {
            console.error('Error loading image:', error);
            if (typeof callback === 'function') {
              callback(null);
            }
          });
      }

    const onClickHandler = async (imageUrl) => {
        console.time('start')
        getBase64Image(imageUrl, (base64) => {
            if (base64) {
                let data = base64.replace(/^data:image\/(png|jpg);base64,/, "");
                console.log('Uint8Array:', base64);
                const image = wasm.greet(data)
                // Create a Blob from the Uint8Array
                const blob = new Blob([image], { type: 'image/png' });
    
                // Create a Data URL from the Blob
                const url = URL.createObjectURL(blob);
                document.getElementById('new').src = url;
                console.log("new image: ", image);
            } else {
                console.err('Error loading the image.');
            }
        })
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