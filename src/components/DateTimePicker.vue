<template>
  <div class="datetime-picker" ref="pickerRef">
    <button class="datetime-toggle" :class="{ active: showPicker }" title="设置截止时间" @click="togglePicker">
      <span class="calendar-icon">📅</span>
    </button>
    <Teleport to="body">
      <div v-if="showPicker" class="datetime-overlay" @click="showPicker = false"></div>
      <div v-if="showPicker" class="datetime-dropdown" :style="dropdownStyle">
        <label class="datetime-label">截止时间</label>
        <input
          ref="dateInput"
          type="datetime-local"
          :value="modelValue"
          @input="$emit('update:modelValue', $event.target.value)"
        />
        <div class="datetime-actions">
          <button v-if="modelValue" class="clear-btn" @click="$emit('update:modelValue', null); showPicker = false">清除</button>
          <button class="confirm-btn" @click="showPicker = false">确定</button>
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup>
import { ref, nextTick } from 'vue'

defineProps({
  modelValue: { type: String, default: null }
})
defineEmits(['update:modelValue'])

const showPicker = ref(false)
const pickerRef = ref(null)
const dateInput = ref(null)
const dropdownStyle = ref({})

async function togglePicker() {
  showPicker.value = !showPicker.value
  if (showPicker.value) {
    await nextTick()
    const rect = pickerRef.value.getBoundingClientRect()
    const dropdownWidth = 220
    let left = rect.left
    // 如果右侧空间不够，向左偏移
    if (left + dropdownWidth > window.innerWidth) {
      left = window.innerWidth - dropdownWidth - 12
    }
    dropdownStyle.value = {
      position: 'fixed',
      top: `${rect.bottom + 6}px`,
      left: `${left}px`,
      zIndex: 1000
    }
  }
}
</script>
