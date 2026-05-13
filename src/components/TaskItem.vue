<template>
  <li
    :class="['task-item', priorityClass, { completing, editing }]"
  >
    <span class="drag-handle" title="拖拽排序">⋮⋮</span>
    <span class="priority-badge">{{ priorityEmoji }}</span>
    <div class="task-body">
      <template v-if="!editing">
        <div class="task-top">
          <span class="task-date">{{ date }}</span>
          <span v-if="dueLabel" :class="['due-tag', dueStatus]">{{ dueLabel }}</span>
        </div>
        <span class="task-content">{{ task.content }}</span>
      </template>
      <template v-else>
        <input
          ref="editInput"
          v-model="editContent"
          class="edit-input"
          @keydown.enter="saveEdit"
          @keydown.escape="cancelEdit"
        />
        <div class="edit-controls">
          <PrioritySelector v-model="editPriority" />
          <input
            type="datetime-local"
            class="edit-due"
            :value="editDueAt"
            @input="editDueAt = $event.target.value"
          />
          <button class="save-btn" @click="saveEdit">✓</button>
          <button class="cancel-btn" @click="cancelEdit">✕</button>
        </div>
      </template>
    </div>
    <div v-if="!editing" class="task-actions">
      <button class="edit-btn" title="编辑" @click="startEdit">✏️</button>
      <button class="complete-btn" title="完成" @click="handleComplete"></button>
    </div>
  </li>
</template>

<script setup>
import { ref, computed, nextTick } from 'vue'
import PrioritySelector from './PrioritySelector.vue'

const props = defineProps({
  task: { type: Object, required: true }
})

const emit = defineEmits(['complete', 'update'])

const completing = ref(false)
const editing = ref(false)
const editContent = ref('')
const editPriority = ref(2)
const editDueAt = ref(null)
const editInput = ref(null)

const date = computed(() => props.task.created_at.split(' ')[0])

const priorityClass = computed(() => {
  if (props.task.priority === 3) return 'priority-high'
  if (props.task.priority === 1) return 'priority-low'
  return 'priority-medium'
})

const priorityEmoji = computed(() => {
  if (props.task.priority === 3) return '🔥'
  if (props.task.priority === 1) return '🌱'
  return '⭐'
})

const dueLabel = computed(() => {
  if (!props.task.due_at) return null
  const due = new Date(props.task.due_at.replace(' ', 'T'))
  const now = new Date()
  const diff = due - now
  if (diff < 0) return '已过期'
  const hours = Math.floor(diff / 3600000)
  if (hours < 1) return `${Math.floor(diff / 60000)}分钟`
  if (hours < 24) return `${hours}小时`
  return `${Math.floor(hours / 24)}天`
})

const dueStatus = computed(() => {
  if (!props.task.due_at) return ''
  const due = new Date(props.task.due_at.replace(' ', 'T'))
  const now = new Date()
  const diff = due - now
  if (diff < 0) return 'overdue'
  if (diff < 3600000) return 'urgent'
  return 'normal'
})

function handleComplete() {
  completing.value = true
  setTimeout(() => emit('complete', props.task.id), 300)
}

async function startEdit() {
  editing.value = true
  editContent.value = props.task.content
  editPriority.value = props.task.priority
  editDueAt.value = props.task.due_at || ''
  await nextTick()
  editInput.value?.focus()
}

function saveEdit() {
  const content = editContent.value.trim()
  if (!content) return
  emit('update', {
    id: props.task.id,
    content,
    priority: editPriority.value,
    dueAt: editDueAt.value || null
  })
  editing.value = false
}

function cancelEdit() {
  editing.value = false
}
</script>
