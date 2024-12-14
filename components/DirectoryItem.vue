<script setup lang="ts">
import type DirectoryItem from '~/models/DirectoryItem';

defineProps<{
    item: DirectoryItem;
    isSelected: boolean;
}>();

defineEmits<{
    'select': [ multiSelect: boolean];
    'open': [item: DirectoryItem];
}>();

</script>

<template>
    <div class="flex items-center p-2 rounded-lg flex-col hover:bg-surface-400"
        :class="{ 'bg-surface-400': isSelected }" @click.stop="$emit('select', false)"
        @click.ctrl.stop.capture="$emit('select', true)" @dblclick.stop="$emit('open', item)">
        <Icon :name="item.isDir ? 'solar:folder-2-bold-duotone' : 'solar:file-bold'"
            :class="item.isDir ? 'text-primary-400' : 'text-primary-200'" class="text-7xl" />
        <div class="overflow-hidden text-nowrap text-ellipsis w-full text-center">
            {{ item.name }}
        </div>
    </div>
</template>