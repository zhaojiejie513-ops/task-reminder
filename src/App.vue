<template>
  <div class="container">
    <header>
      <div class="header-top">
        <div class="header-icon">&#9745;</div>
        <div class="header-text">
          <h1>待办事项</h1>
          <span class="header-date">{{ today }}</span>
        </div>
      </div>
      <div v-if="tasks.length > 0" class="task-count">
        <span class="count-badge">{{ tasks.length }}</span>
        <span class="count-label">项待办</span>
      </div>
    </header>
    <div class="input-area">
      <div class="input-row">
        <input
          v-model="newTask"
          type="text"
          placeholder="添加新任务..."
          autocomplete="off"
          @keydown.enter="addTask"
        />
        <button class="add-btn" @click="addTask">
          <span class="add-icon">+</span>
        </button>
      </div>
      <div class="input-options">
        <PrioritySelector v-model="newPriority" />
        <DateTimePicker v-model="newDueAt" />
        <span v-if="newDueAt" class="due-preview">{{ formatDuePreview }}</span>
      </div>
    </div>
    <draggable
      v-model="tasks"
      tag="ul"
      class="task-list"
      item-key="id"
      handle=".drag-handle"
      animation="200"
      @end="onDragEnd"
    >
      <template #item="{ element }">
        <TaskItem
          :task="element"
          @complete="completeTask"
          @update="updateTask"
        />
      </template>
    </draggable>
    <div v-if="tasks.length === 0" class="empty-state">
      <div class="empty-icon">&#9996;</div>
      <p class="empty-text">没有待办事项</p>
      <p class="empty-sub">享受今天吧</p>
    </div>
    <div v-if="completedTasks.length > 0" class="completed-section">
      <button class="completed-toggle" @click="showCompleted = !showCompleted">
        <span class="toggle-arrow" :class="{ open: showCompleted }">&#9654;</span>
        已完成 ({{ completedTasks.length }})
        <button v-if="showCompleted" class="clear-all-btn" @click.stop="clearCompleted">清空</button>
      </button>
      <ul v-if="showCompleted" class="completed-list">
        <CompletedItem
          v-for="task in completedTasks"
          :key="task.id"
          :task="task"
          @delete="deleteCompleted"
          @restore="restoreTask"
        />
      </ul>
    </div>
    <div class="bg-circle bg-circle-1"></div>
    <div class="bg-circle bg-circle-2"></div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import draggable from 'vuedraggable'
import TaskItem from './components/TaskItem.vue'
import CompletedItem from './components/CompletedItem.vue'
import PrioritySelector from './components/PrioritySelector.vue'
import DateTimePicker from './components/DateTimePicker.vue'

const tasks = ref([])
const completedTasks = ref([])
const newTask = ref('')
const newPriority = ref(2)
const newDueAt = ref(null)
const showCompleted = ref(false)

const today = computed(() => {
  const d = new Date()
  const weekdays = ['日', '一', '二', '三', '四', '五', '六']
  return `${d.getMonth() + 1}月${d.getDate()}日 周${weekdays[d.getDay()]}`
})

const formatDuePreview = computed(() => {
  if (!newDueAt.value) return ''
  const d = new Date(newDueAt.value)
  return `${d.getMonth() + 1}/${d.getDate()} ${d.getHours()}:${String(d.getMinutes()).padStart(2, '0')}`
})

async function loadTasks() {
  tasks.value = await invoke('get_tasks')
  completedTasks.value = await invoke('get_completed_tasks')
}

async function addTask() {
  const content = newTask.value.trim()
  if (!content) return
  const task = await invoke('add_task', { content, priority: newPriority.value })
  if (task && newDueAt.value) {
    await invoke('update_task', { id: task.id, dueAt: newDueAt.value })
  }
  newTask.value = ''
  newDueAt.value = null
  newPriority.value = 2
  await loadTasks()
}

async function completeTask(id) {
  await invoke('complete_task', { id })
  await loadTasks()
}

async function updateTask({ id, content, priority, dueAt }) {
  await invoke('update_task', { id, content, priority, dueAt })
  await loadTasks()
}

async function deleteCompleted(id) {
  await invoke('delete_completed_task', { id })
  await loadTasks()
}

async function restoreTask(id) {
  await invoke('restore_task', { id })
  await loadTasks()
}

async function clearCompleted() {
  await invoke('clear_completed_tasks')
  await loadTasks()
}

async function onDragEnd() {
  const ids = tasks.value.map(t => t.id)
  await invoke('reorder_tasks', { ids })
}

onMounted(loadTasks)
</script>
