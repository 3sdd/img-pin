<script setup lang="ts">
import { appWindow } from "@tauri-apps/api/window";
import { onMounted, ref } from "vue";

const alwaysOnTop = ref(false);
function onPinClick() {
  alwaysOnTop.value = !alwaysOnTop.value;
  appWindow.setAlwaysOnTop(alwaysOnTop.value);
}
function onMinimizeClick() {
  appWindow.minimize();
}

function onMaximizeClick() {
  appWindow.toggleMaximize();
}
function onCloseClick() {
  appWindow.close();
}

const fileMenu = ["Open", "OpenRecent"];
</script>

<template>
  <div data-tauri-drag-region class="titlebar">
    <div class="titlebar-left flex-none">
      <button class="bg-gray-200 px-2">File</button>
      <button class="bg-gray-200 px-2">Edit</button>
    </div>

    <div
      x-show="dropdownOpen"
      class="absolute left-0 top-5 mt-2 w-48 bg-white rounded-md overflow-hidden shadow-xl z-20"
    >
      <a
        href="#"
        class="block px-4 py-2 text-sm text-gray-800 border-b hover:bg-gray-200"
        >Open</a
      >
      <a
        href="#"
        class="block px-4 py-2 text-sm text-gray-800 border-b hover:bg-gray-200"
        >Open Recent</a
      >
      <a
        href="#"
        class="block px-4 py-2 text-sm text-gray-800 border-b hover:bg-gray-200"
        >(seperator)</a
      >
    </div>

    <div class="titlebar-center flex-grow" data-tauri-drag-region>
      <span class="px-2" data-tauri-drag-region>タイトル</span>
    </div>
    <div class="titlebar-right flex-none">
      <div class="titlebar-button" id="titlebar-pin" @click="onPinClick">
        <img
          src="https://api.iconify.design/mdi:pin-off.svg"
          alt="pin-off"
          v-if="alwaysOnTop"
        />
        <img src="https://api.iconify.design/mdi:pin.svg" alt="pin-on" v-else />
      </div>
      <div
        class="titlebar-button"
        id="titlebar-minimize"
        @click="onMinimizeClick"
      >
        <img
          src="https://api.iconify.design/mdi:window-minimize.svg"
          alt="minimize"
        />
      </div>
      <div
        class="titlebar-button"
        id="titlebar-maximize"
        @click="onMaximizeClick"
      >
        <img
          src="https://api.iconify.design/mdi:window-maximize.svg"
          alt="maximize"
        />
        <!-- <img
        src="https://api.iconify.design/mdi:window-restore.svg"
        alt="restore"
        /> -->
      </div>
      <div class="titlebar-button" id="titlebar-close" @click="onCloseClick">
        <img src="https://api.iconify.design/mdi:close.svg" alt="close" />
      </div>
    </div>
  </div>
</template>

<style>
.titlebar {
  height: 30px;
  background: #329ea3;
  user-select: none;
  display: flex;

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
