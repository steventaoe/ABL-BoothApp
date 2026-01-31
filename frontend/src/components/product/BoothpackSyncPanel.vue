<template>
  <div class="sync-container">
    <div class="section-header" @click="isCollapsed = !isCollapsed">
      <h2>制品数据包（.boothpack）</h2>
      <n-button text class="toggle-btn">
        {{ isCollapsed ? '展开' : '折叠' }}
      </n-button>
    </div>

    <transition name="expand">
      <div v-show="!isCollapsed" class="section-body">
        <n-alert type="info" :bordered="false" class="info-alert">
          <div class="info-text">
            你可以导出当前制品库为 <code>.boothpack</code> 作为备份，或在其他设备导入。
            <br />
            <strong>注意：</strong>导入会覆盖编号相同的制品，建议先导出备份。
          </div>
        </n-alert>

        <n-alert
          v-if="syncMessage"
          type="success"
          :bordered="false"
          class="sync-alert"
          closable
          @close="syncMessage = ''"
        >
          {{ syncMessage }}
        </n-alert>
        <n-alert
          v-if="syncError"
          type="error"
          :bordered="false"
          class="sync-alert"
          closable
          @close="syncError = ''"
        >
          {{ syncError }}
        </n-alert>

        <div class="sync-controls">
          <n-button size="large" type="success" :loading="isExporting" @click="handleExport">
            导出 .boothpack
          </n-button>

          <n-button size="large" type="info" :loading="isImporting" @click="triggerImport">
            导入 .boothpack
          </n-button>

          <input
            ref="importFileInputRef"
            type="file"
            class="hidden-input"
            accept=".zip,.boothpack"
            @change="handleImportFile"
          />
        </div>

        <div
          class="drop-zone"
          :class="{ 'is-dragging': isDragging }"
          @dragenter.prevent="onDragEnter"
          @dragover.prevent="onDragOver"
          @dragleave.prevent="onDragLeave"
          @drop.prevent="onDrop"
        >
          <div class="drop-zone-content">
            <span class="drop-zone-icon">📂</span>
            <span class="drop-zone-text">将 .boothpack 或 .zip 文件拖拽到此处导入</span>
          </div>
        </div>
      </div>
    </transition>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onBeforeUnmount, h } from 'vue'
import { useSyncStore } from '@/stores/syncStore'
import { NButton, NAlert, useDialog, useMessage } from 'naive-ui'

const emit = defineEmits(['imported'])

const syncStore = useSyncStore()
const dialog = useDialog()
const message = useMessage()

const isCollapsed = ref(false)

const importFileInputRef = ref(null)
const syncMessage = ref('')
const syncError = ref('')

const isExporting = computed(() => syncStore.isExporting)
const isImporting = computed(() => syncStore.isImporting)

const isDragging = ref(false)
let dragCounter = 0
let tauriUnlisten = null
let globalDropCleanup = null

function clearSyncHints() {
  syncMessage.value = ''
  syncError.value = ''
}

function isAllowedPackName(name) {
  const lowered = String(name || '').toLowerCase()
  return lowered.endsWith('.boothpack') || lowered.endsWith('.zip')
}

function rejectInvalidFile(name) {
  syncError.value = '请导入 .boothpack 或 .zip 文件'
  message.warning(`文件 "${name || 'unknown'}" 不是有效的 .boothpack/.zip`, {
    duration: 4000,
    closable: true
  })
}

function triggerImport() {
  clearSyncHints()
  importFileInputRef.value?.click?.()
}

async function handleImportFile(event) {
  const file = event.target?.files?.[0]
  if (!file) return
  await confirmAndImport({ kind: 'file', file, displayName: file.name })
  event.target.value = ''
}

async function handleExport() {
  clearSyncHints()
  try {
    const { filename } = await syncStore.exportProducts()

    // ✅ 这里建议做“移动端提示”：告诉用户需要选择保存/分享路径
    syncMessage.value = `已导出：${filename}`
    message.success(`已成功导出制品包：${filename}`, { duration: 5000, closable: true })
  } catch (error) {
    const msg = error?.message || '导出失败'
    syncError.value = msg
    message.error(`导出失败：${msg}`, { duration: 5000, closable: true })
  }
}

async function confirmAndImport({ kind, file, path, displayName }) {
  const name =
    displayName ||
    (kind === 'path' ? String(path).split(/[/\\]/).pop() : file?.name) ||
    'unknown'

  if (!isAllowedPackName(name)) {
    rejectInvalidFile(name)
    return
  }

  if (kind === 'file' && file?.size != null && file.size > 1024 * 1024 * 1024) {
    syncError.value = '文件过大（>1GB），请确认是否选择正确的数据包'
    message.warning('文件过大，可能不是正确的制品包', { duration: 5000, closable: true })
    return
  }

  dialog.warning({
    title: kind === 'path' ? '检测到文件拖入' : '确认导入',
    content: () =>
      h('div', { style: 'white-space: pre-line;' }, [
        `文件名：${name}`,
        '\n\n',
        '确定要导入吗？这将覆盖或更新现有的商品数据。',
        '\n',
        '建议导入前先导出当前数据作为备份。'
      ]),
    positiveText: '确定导入',
    negativeText: '取消',
    onPositiveClick: async () => {
      if (isImporting.value) {
        message.info('正在导入中，请稍候…', { duration: 2000, closable: true })
        return false
      }

      clearSyncHints()
      try {
        let result
        if (kind === 'file') {
          result = await syncStore.importProducts(file)
        } else {
          result = await syncStore.importProductsFromPath(path, true)
        }

        const count = result?.count ?? '若干'
        syncMessage.value = `导入成功，更新 ${count} 条制品。`
        message.success(`导入成功！已更新 ${count} 条制品数据`, { duration: 5000, closable: true })

        // ✅ 通知父组件刷新列表
        emit('imported')
      } catch (error) {
        const msg = error?.message || '导入失败'
        syncError.value = msg
        message.error(`导入失败：${msg}`, { duration: 5000, closable: true })
      }
    }
  })
}

/* Web 拖拽 */
function onDragEnter(event) {
  event.stopPropagation()
  dragCounter += 1
  isDragging.value = true
}
function onDragOver() {
  isDragging.value = true
}
function onDragLeave(event) {
  event.stopPropagation()
  dragCounter = Math.max(0, dragCounter - 1)
  if (dragCounter === 0) isDragging.value = false
}

function extractFirstFileFromDataTransfer(dt) {
  if (!dt) return null
  const f0 = dt.files?.[0]
  if (f0) return f0
  const items = dt.items
  if (items && items.length) {
    for (const it of items) {
      if (it.kind === 'file') {
        const f = it.getAsFile?.()
        if (f) return f
      }
    }
  }
  return null
}

async function onDrop(event) {
  isDragging.value = false
  dragCounter = 0
  const file = extractFirstFileFromDataTransfer(event.dataTransfer)
  if (file) await confirmAndImport({ kind: 'file', file, displayName: file.name })
  event.dataTransfer?.clearData?.()
}

/* Tauri file drop */
function isTauriEnv() {
  return typeof window !== 'undefined' && !!window.__TAURI_INTERNALS__
}

async function setupTauriFileDrop() {
  if (!isTauriEnv()) return

  try {
    const { listen } = await import('@tauri-apps/api/event')
    const unlistenFns = []

    const handlePaths = async (paths) => {
      const first = Array.isArray(paths) ? paths[0] : paths
      if (!first || typeof first !== 'string') return
      const fileName = first.split(/[/\\]/).pop() || 'import.boothpack'
      await confirmAndImport({ kind: 'path', path: first, displayName: fileName })
    }

    try {
      const unlisten = await listen('boothpack-file-drop', (event) => handlePaths(event.payload))
      unlistenFns.push(unlisten)
    } catch {}

    try {
      const unlisten = await listen('tauri://file-drop', (event) => {
        const paths = event.payload?.paths || event.payload
        handlePaths(paths)
      })
      unlistenFns.push(unlisten)
    } catch {}

    tauriUnlisten = () => {
      unlistenFns.forEach((fn) => typeof fn === 'function' && fn())
      unlistenFns.length = 0
    }
  } catch (err) {
    console.error('[Tauri] setup file-drop failed:', err)
  }
}

onMounted(async () => {
  // ✅ 仅 Tauri 中阻止“拖拽打开文件替换整个 webview”
  if (isTauriEnv()) {
    const handleWindowDragOver = (e) => e.preventDefault()
    const handleWindowDrop = (e) => e.preventDefault()
    window.addEventListener('dragover', handleWindowDragOver)
    window.addEventListener('drop', handleWindowDrop)
    globalDropCleanup = () => {
      window.removeEventListener('dragover', handleWindowDragOver)
      window.removeEventListener('drop', handleWindowDrop)
    }
  }

  await setupTauriFileDrop()
})

onBeforeUnmount(() => {
  if (typeof tauriUnlisten === 'function') tauriUnlisten()
  if (typeof globalDropCleanup === 'function') globalDropCleanup()
})
</script>

<style scoped>
.sync-container {
  margin-bottom: 2rem;
}

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  cursor: pointer;
  user-select: none;
  padding: 0.75rem 1rem;
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  transition: all 0.2s ease;
  margin-bottom: 0.5rem;
}

.section-header:hover {
  background: var(--hover-bg-color, var(--card-bg-color));
  border-color: var(--accent-color);
}

.section-header h2 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--accent-color);
  font-weight: 600;
}

.toggle-btn {
  font-size: 0.9rem;
  padding: 0.25rem 0.75rem;
  min-width: auto;
  color: var(--accent-color);
}

.expand-enter-active,
.expand-leave-active {
  transition: all 0.3s ease;
  overflow: hidden;
}
.expand-enter-from,
.expand-leave-to {
  opacity: 0;
  max-height: 0;
}
.expand-enter-to,
.expand-leave-from {
  opacity: 1;
  max-height: 2000px;
}

.section-body {
  background: var(--card-bg-color);
  border: 2px solid var(--border-color);
  border-radius: 8px;
  padding: 1.25rem;
}

.info-alert {
  margin-bottom: 12px;
}

.sync-alert {
  margin-top: 12px;
}

.sync-controls {
  display: flex;
  gap: 12px;
  flex-wrap: wrap;
  align-items: center;
  margin-top: 12px;
  margin-bottom: 12px;
}

.hidden-input {
  display: none;
}

.drop-zone {
  border: 2px dashed var(--border-color);
  border-radius: 8px;
  padding: 12px 14px;
  background: var(--bg-color);
  transition: border-color 0.2s ease, background-color 0.2s ease;
}

.drop-zone.is-dragging {
  border-color: var(--accent-color);
  background: var(--accent-color-light);
}

.drop-zone-content {
  display: flex;
  align-items: center;
  gap: 10px;
  color: var(--text-muted);
  font-size: 0.95rem;
}

.drop-zone-icon {
  font-size: 1.2rem;
}
</style>
