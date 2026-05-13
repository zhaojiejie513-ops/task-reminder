<template>
  <li class="completed-item">
    <span class="completed-content">{{ task.content }}</span>
    <span class="completed-time">{{ completedTime }}</span>
    <button class="restore-btn" title="恢复" @click="$emit('restore', task.id)">↩</button>
    <button class="delete-btn" title="永久删除" @click="$emit('delete', task.id)">&times;</button>
  </li>
</template>

<script setup>
import { computed } from 'vue'

const props = defineProps({
  task: { type: Object, required: true }
})
defineEmits(['delete', 'restore'])

const completedTime = computed(() => {
  if (!props.task.completed_at) return ''
  return props.task.completed_at.split(' ')[0]
})
</script>
