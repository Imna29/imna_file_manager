<script lang="ts" setup>
import * as path from '@tauri-apps/api/path';

navigateTo(await path.homeDir());

</script>

<template>
  <div>
    <NuxtPage/>
  </div>
</template>
