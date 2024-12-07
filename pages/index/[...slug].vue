<script setup lang="ts">
import { invoke } from '@tauri-apps/api/core';

const currentPath = useRoute().path;

function handleNavigate(path: string) {
    navigateTo(path);
}

function handleFileOpen(path: string) {
    invoke('open_file', { path });
}

</script>

<template>
    <div class="h-dvh">
      <PathView :directory="currentPath" @navigate="handleNavigate" @open-file="handleFileOpen" class="h-full"/>
    </div>
</template>