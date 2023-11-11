import React from 'react';
import * as resim from "resim";
import { createRoot } from 'react-dom/client';
import cat from './asset/cat.jpeg'

const Component = () => {

  const onClickHandler = async () => {
    console.time('start')
    var canvas = document.getElementById("canvas");
    var ctx = canvas.getContext("2d");

    ctx.drawImage(document.getElementById('dice'), 0, 0);
    const imageResim = resim.convertToGrayscale(canvas, ctx)
    resim.placeImage(canvas, ctx, imageResim);

    console.timeEnd('start')
  }
  return (
    <>
      <div style={{ display: 'flex' }} >
        <button onClick={() => onClickHandler()} >Convert</button>
      </div>
      <div>
        <img id='dice' src={cat} />
        <canvas id='canvas' width={1000} height={1000} />
      </div>
    </>
  );
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component />);