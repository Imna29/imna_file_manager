<script setup lang="ts">
import {invoke} from '@tauri-apps/api/core';

const currentPath = useRoute().path;

function handleNavigate(path: string) {
  navigateTo(path);
}


function handleFileOpen(path: string) {
  invoke('open_file', {path});
}

const menuItems = useRoute().path === "/" ? [{label: ""}] : useRoute().path.split('/').map((item) => {
  return {
    label: item,
  }
});

function navigateToPathFromMenu(path: string) {
  const allPaths = useRoute().path.split('/');
  const newPath = allPaths.slice(0, allPaths.indexOf(path) + 1).join('/');
  handleNavigate(newPath);
}

</script>

<template>
  <div class="flex flex-col h-full min-h-0">
    <div class="flex-shrink-0">
      <div class="flex flex-row items-center w-full gap-2 bg-surface-900 h-full">
        <div class="flex flex-row">
          <button class="hover:bg-surface-400 rounded-lg flex flex-row items-center h-full" @click="useRouter().back()">
            <Icon name="solar:arrow-left-line-duotone" size="2em" mode="css" class=""/>
          </button>
          <button class=" hover:bg-surface-400 rounded-lg flex flex-row items-center  h-full"
                  @click="useRouter().forward()">
            <Icon name="solar:arrow-right-line-duotone" size="2em" mode="css" class=""/>
          </button>
        </div>
        <div class="w-full">
          <Breadcrumb :model="menuItems">
            <template #item="{item}">
              <div v-if="item.label!==''" class="cursor-pointer" @click="navigateToPathFromMenu(item.label as string)">
                {{ item.label }}
              </div>
              <div v-else>
                <Icon name="solar:home-outline" class="cursor-pointer" @click="navigateToPathFromMenu('')"
                ></Icon>
              </div>
            </template>
          </Breadcrumb>
        </div>
      </div>
    </div>
    <div class="flex w-dvw flex-1 h-full min-h-0">
      <div class="h-full bg-surface-900 max-w-64 col-span-2 w-32 flex-1">
        <LazySidebar></LazySidebar>
      </div>
      <div class="h-full min-w-0 flex-1 w-full bg-surface-800">
        <PathView :directory="currentPath" @navigate="handleNavigate" @open-file="handleFileOpen" class="h-full "/>
      </div>
    </div>
  </div>

</template>