import * as wasm from "./colorblock.js";

window.wasm = wasm;

const base64_arraybuffer = async (data) => {
  // Use a FileReader to generate a base64 data URI
  const base64url = await new Promise((r) => {
    const reader = new FileReader();
    reader.onload = () => r(reader.result);
    reader.readAsDataURL(new Blob([data]));
  });

  /*
  The result looks like 
  "data:application/octet-stream;base64,<your base64 data>", 
  so we split off the beginning:
  */
  return base64url.substring(base64url.indexOf(",") + 1);
};

window.update = async () => {
  let e = document.getElementById("color-picker");
  let list = wasm.color_search(e.value);
  let top = list.slice(0, 10);
  let html = "";
  for (let item of top) {
    console.log(item);
    let base64 = await base64_arraybuffer(new Uint8Array(item.bin));
    let dataurl = `data:image/png;base64,${base64}`;
    console.log(dataurl);
    html += `<div class="listItem"><img style="image-rendering: pixelated;" src="${dataurl}"/><h3>${item.name}</h3></div>`;
  }
  document.querySelector(".output").innerHTML = html;
  console.log(list);
};
