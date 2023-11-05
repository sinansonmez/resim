import React, { useEffect } from 'react';
import * as wasm from "hello-wasm-pack";
import { createRoot } from 'react-dom/client';
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
        let data = base64.replace(/^data:image\/(png|jpeg);base64,/, "");
        const image = wasm.grayscale(data)
        // Create a canvas element
        const canvas = document.createElement('canvas');

        // Set the canvas dimensions to match the ImageData's dimensions
        canvas.width = image.width;
        canvas.height = image.height;

        // Get the 2D rendering context
        const ctx = canvas.getContext('2d');

        // Use putImageData to render the ImageData
        ctx.putImageData(image, 0, 0);

        // Append the canvas to the DOM or do other operations
        document.body.appendChild(canvas);
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
      </div>
    </>
  );
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component />);