<script setup lang="ts">
import * as path from '@tauri-apps/api/path';

const home = await path.homeDir();
const documents = await path.documentDir();
const downloads = await path.downloadDir();
const videos = await path.videoDir();
const audio = await path.audioDir();
</script>

<template>
  <div class="flex flex-col overflow-y-auto px-2 cursor-pointer">
    <NuxtLink :to="home">
      <div class="p-2 rounded-lg hover:bg-surface-400">Home</div>
    </NuxtLink>
    <NuxtLink :to="documents">
      <div class="p-2 rounded-lg hover:bg-surface-400">Documents</div>
    </NuxtLink>
    <NuxtLink :to="downloads">
      <div class="p-2 rounded-lg hover:bg-surface-400">Downloads</div>
    </NuxtLink>
    <NuxtLink :to="videos">
      <div class="p-2 rounded-lg hover:bg-surface-400">Videos</div>
    </NuxtLink>
    <NuxtLink :to="audio">
      <div class="p-2 rounded-lg hover:bg-surface-400">Audio</div>
    </NuxtLink>
  </div>
</template>

