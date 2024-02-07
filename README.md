<p align="center">
  <!-- <img src="https://raw.githubusercontent.com/BLaZeKiLL/VloxyEngine/main/.github/assets/vloxy_logo.svg" width=256> -->
  <h1 align="center">WebRay</h1>
</p>

<p align="center">
    <img width=96 src="./docs/icons/logo_rust.png">
    <img width=128 src="./docs/icons/logo_web_gpu.svg">
    <img width=96 src="./docs/icons/logo_web_assembly.svg">
    <img width=72 src="./docs/icons/logo_svelte.svg">
</p>

<p align="center">
Webray is a WebGPU powered ray tracer + Svelte based editor
</p>

> :warning: **WebGPU**: Is experimental api. Webray is tested using the latest desktop chrome version. system may slow down or crash during rendering or while working with large scenes (Demo 02), please use webray at your own risk.

## Editor Documentation
The editor is available [here](https://blazekill.github.io/webray/) along [Demo Scene 01](https://github.com/BLaZeKiLL/webray/blob/main/src/data/demo_01.scene.json) and [Demo Scene 02](https://github.com/BLaZeKiLL/webray/blob/main/src/data/demo_02.scene.json) which can be imported.

### Tools 
- <img align="center" src="https://api.iconify.design/iconamoon/3d-light.svg?color=%23888888"> Objects in the current scene
- <img align="center" src="https://api.iconify.design/uil/image-download.svg?color=%23888888"> Download rendered image
- <img align="center" src="https://api.iconify.design/material-symbols/imagesmode-outline-rounded.svg?color=%23888888"> Render the current scene as an image
- <img align="center" src="https://api.iconify.design/material-symbols:android-camera-outline.svg?color=%23888888"> Camera Settings
- <img align="center" src="https://api.iconify.design/cil/tv.svg?color=%23888888"> Render Settings
- <img align="center" src="https://api.iconify.design/icon-park-outline/material.svg?color=%23888888"> Materials in the current scene
- <img align="center" src="https://api.iconify.design/gridicons/fullscreen.svg?color=%23888888"> Enter full screen image
- <img align="center" src="https://api.iconify.design/gridicons/fullscreen-exit.svg?color=%23888888"> Exit full screen image
- <img align="center" src="https://api.iconify.design/material-symbols/delete-forever-outline-rounded.svg?color=%23888888"> Delete item
- <img align="center" src="https://api.iconify.design/material-symbols/file-save-outline-rounded.svg?color=%23888888"> Download scene json file
- <img align="center" src="https://api.iconify.design/material-symbols/file-open-outline-rounded.svg?color=%23888888"> Import scene json file

## WebRay Renderer
The renderer is implemented in rust and compiled to wasm with the core kernel implemented as a wgsl compute shader using wgpu.