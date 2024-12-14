<script setup lang="ts">
import {ref} from 'vue';
import {invoke} from '@tauri-apps/api/core';
import type DirectoryItem from '~/models/DirectoryItem';
import DirectoryItemComponent from './DirectoryItem.vue';

const props = defineProps<{
  directory: string;
}>();

const emit = defineEmits<{
  'navigate': [path: string],
  'open-file': [path: string]
}>();

const isLoading = ref(false);
const itemsRef = ref<Array<{ isSelected: boolean; item: DirectoryItem }>>([]);
const selectedIndex = ref<number>(-1);
const containerRef = ref<HTMLElement | null>(null);
const itemWidth = 9 * parseFloat(getComputedStyle(document.documentElement).fontSize);
const {width, height} = useElementSize(containerRef)

const numItemsPerRow = computed(() =>
    Math.max(Math.floor(width.value / itemWidth), 1)
);

// Convert items to rows for grid layout
const rows = computed(() => {
  const items = itemsRef.value;
  const itemsPerRow = numItemsPerRow.value;
  const rows = [];

  for (let i = 0; i < items.length; i += itemsPerRow) {
    rows.push(items.slice(i, i + itemsPerRow));
  }
  return rows;
});

async function loadDirectory(path: string) {
  try {
    isLoading.value = true;
    let items: DirectoryItem[] = await invoke('get_directory_content', {directory: path});

    const sortedItems = items.sort((a, b) => {
      if (a.isDir && !b.isDir) return -1;
      if (!a.isDir && b.isDir) return 1;
      return a.name.localeCompare(b.name);
    });

    itemsRef.value = sortedItems.map(i => ({isSelected: false, item: i}));
  } catch (error) {
    console.error('Failed to load directory:', error);
  } finally {
    isLoading.value = false;
  }
}

function handleSelect(item: DirectoryItem, multiSelect: boolean, index: number) {
  if (!multiSelect) clearSelection();
  selectedIndex.value = index;
  itemsRef.value[index].isSelected = !itemsRef.value[index].isSelected;
}

function handleOpen(item: DirectoryItem) {
  console.log("open");
  if (item.isDir) emit('navigate', item.path);
  else emit('open-file', item.path);
}

function clearSelection() {
  itemsRef.value.forEach(i => i.isSelected = false);
}

// Keyboard navigation
function handleKeyNavigation(e: KeyboardEvent) {
  if (!itemsRef.value.length) return;
  //detect if ctrl key is pressed
  const isCtrlPressed = e.ctrlKey || e.metaKey;


  const currentIdx = selectedIndex.value;

  switch (e.key) {
    case 'ArrowRight':
      selectedIndex.value = Math.min(currentIdx + 1, itemsRef.value.length - 1);
      break;
    case 'ArrowLeft':
      selectedIndex.value = Math.max(currentIdx - 1, 0);
      break;
    case 'ArrowUp':
      selectedIndex.value = Math.max(currentIdx - numItemsPerRow.value, 0);
      break;
    case 'ArrowDown':
      selectedIndex.value = Math.min(currentIdx + numItemsPerRow.value, itemsRef.value.length - 1);
      break;
    case ' ':
      itemsRef.value[currentIdx].isSelected = !itemsRef.value[currentIdx].isSelected;
      break;
  }

  if (selectedIndex.value !== currentIdx) {
    if (!isCtrlPressed) {
      clearSelection();
      itemsRef.value[selectedIndex.value].isSelected = true;
    }
  }
}

watchEffect(() => {
  loadDirectory(props.directory);
});


</script>

<template>
  <div ref="containerRef" class="w-full flex-1 p-4" @keydown="handleKeyNavigation" role="grid" tabindex="0"
       @keyup.enter="handleOpen(itemsRef[selectedIndex].item)">
    <template v-if="isLoading">
      <div class="w-full text-center">Loading...</div>
    </template>
    <template v-else>
      <VirtualScroller :items="rows" :itemSize="[112, itemWidth]" orientation="both" class="h-dvh w-full"
                       :scrollHeight="height + 'px'" :scrollWidth="width + 'px'" :autoSize="true">
        <template v-slot:item="{ item, options }">
          <div :class="['flex items-center p-2', { 'bg-surface-100 dark:bg-surface-700': options.odd }]"
               style="height: 112px">
            <template v-for="(el, index) of item" :key="index">
              <div :style="{ width: `${itemWidth}px`, height: '112px' }" class="flex items-center justify-center">
                <DirectoryItemComponent :item="el.item" :isSelected="el.isSelected" class="w-full h-full "
                                        :class="{'border-b-4' : options.index * numItemsPerRow + index === selectedIndex}"
                                        @select="(multiSelect) => handleSelect(el.item, multiSelect, options.index * numItemsPerRow + index)"
                                        @open="handleOpen"/>
              </div>
            </template>
          </div>
        </template>
      </VirtualScroller>
    </template>
  </div>
</template>

<style>
.p-virtualscroller {
  @apply w-full;
}

.p-virtualscroller-content {
  @apply w-full;
}
</style>