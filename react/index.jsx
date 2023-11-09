import React from 'react';
import * as resim from "resim";
import { createRoot } from 'react-dom/client';
import dice from './asset/dice.png'

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
        <button onClick={() => onClickHandler(dice)} >Hello from React!</button>
      </div>
      <div>
        <img id='dice' src={dice} />
        <canvas id='canvas' width={800} height={600} />
      </div>
    </>
  );
}

// Render your React component instead
const root = createRoot(document.getElementById('app'));
root.render(<Component />);