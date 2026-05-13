<template>
  <div class="priority-selector">
    <button
      v-for="level in levels"
      :key="level.value"
      :class="['priority-emoji', { active: modelValue === level.value }]"
      :title="level.label"
      @click="$emit('update:modelValue', level.value)"
    >{{ level.emoji }}</button>
  </div>
</template>

<script setup>
defineProps({
  modelValue: { type: Number, default: 2 }
})
defineEmits(['update:modelValue'])

const levels = [
  { value: 3, label: '紧急', emoji: '🔥' },
  { value: 2, label: '普通', emoji: '⭐' },
  { value: 1, label: '轻松', emoji: '🌱' }
]
</script>
