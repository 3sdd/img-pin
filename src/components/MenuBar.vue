<script setup lang="ts">
import { appWindow } from '@tauri-apps/api/window'
import { ref } from 'vue';

const alwaysOnTop=ref(false);
function onPinClick(){
    alwaysOnTop.value=!alwaysOnTop.value
    appWindow.setAlwaysOnTop(alwaysOnTop.value);
}
function onMinimizeClick(){
    appWindow.minimize()
}

function onMaximizeClick(){
    appWindow.toggleMaximize()
}
function onCloseClick(){
    appWindow.close()
}


</script>

<template>
    <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-button" id="titlebar-pin" @click="onPinClick">
    <img
    src="https://api.iconify.design/mdi:pin.svg"
    alt="pin"
    />
</div>
  <div class="titlebar-button" id="titlebar-minimize" @click="onMinimizeClick">
      <img
      src="https://api.iconify.design/mdi:window-minimize.svg"
      alt="minimize"
      />
  </div>
  <div class="titlebar-button" id="titlebar-maximize" @click="onMaximizeClick">
      <img
      src="https://api.iconify.design/mdi:window-maximize.svg"
      alt="maximize"
      />
  </div>
  <div class="titlebar-button" id="titlebar-close" @click="onCloseClick">
      <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
  </div>
  </div>
</template>


<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;
  justify-content: flex-end;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
}
.titlebar-button {
  display: inline-flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
}
.titlebar-button:hover {
  background: #5bbec3;
}
</style>