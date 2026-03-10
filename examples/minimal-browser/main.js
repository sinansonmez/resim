import init, {
  brightnessImageData,
  readImageDataFromCanvas,
  writeImageDataToCanvas,
} from "@sinansonmez/resim";

const canvas = document.getElementById("preview");
const ctx = canvas.getContext("2d");

ctx.fillStyle = "#6f8f4f";
ctx.fillRect(0, 0, canvas.width, canvas.height);

await init();

const imageData = readImageDataFromCanvas(canvas, ctx);
const next = brightnessImageData(imageData, 25);
writeImageDataToCanvas(ctx, next);
