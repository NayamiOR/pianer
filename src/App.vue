<script lang="ts" setup>
import {Ref, ref, onMounted} from "vue";
import {message, open} from "@tauri-apps/plugin-dialog";
import OperationZone from "./components/OperationZone.vue";
import SettingsDialog from "./components/SettingsDialog.vue";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";
import { Files, Moon, Sunny, Setting } from '@element-plus/icons-vue';

const currentDir: Ref<string | null> = ref(null);
const recursiveRadio: Ref<string> = ref("unrecursive");
const fileNames: Ref<string | null> = ref(null);
const fileNameList: Ref<string[]> = ref([]);

const progressPercentage: Ref<number> = ref(0);
const progressStatus: Ref<string> = ref("");
const isDarkMode: Ref<boolean> = ref(false);

// 设置相关
const settingsDialogVisible = ref(false);
const settings = ref({
  fileExtensions: '',  // 文件扩展名过滤
  maxFileSize: 100,    // 最大文件大小 (MB)
  skipHiddenFiles: true, // 跳过隐藏文件
});

function openSettings() {
  settingsDialogVisible.value = true;
}

interface Settings {
  fileExtensions: string;
  maxFileSize: number;
  skipHiddenFiles: boolean;
}

function handleSaveSettings(newSettings: Settings) {
  // 保存设置逻辑
  settings.value = newSettings;
  // 这里可以添加调用后端API保存设置的代码
  console.log('保存设置:', newSettings);
}

// 初始化主题
onMounted(() => {
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'dark') {
    enableDarkMode();
  }
});

// 切换暗黑模式
function toggleDarkMode() {
  if (isDarkMode.value) {
    disableDarkMode();
  } else {
    enableDarkMode();
  }
}

// 启用暗黑模式
function enableDarkMode() {
  document.documentElement.classList.add('dark');
  localStorage.setItem('theme', 'dark');
  isDarkMode.value = true;
}

// 禁用暗黑模式
function disableDarkMode() {
  document.documentElement.classList.remove('dark');
  localStorage.setItem('theme', 'light');
  isDarkMode.value = false;
}

async function openDir() {
  const path = await open(
      {
        directory: true,
        multiple: false,
        title: "选择目录",
      }
  )
  currentDir.value = path;
  try {
    await invoke("set_source_folder", {sourcePath: path, recursive: recursiveRadio.value});
  } catch (e) {
    await message('选择目录失败，原因：' + e);
  }
}


function getFileNames() {
  if (fileNames.value == null) {
    fileNames.value = "";
  }
  fileNameList.value = fileNames.value.split("\n");
  fileNameList.value = fileNameList.value.map((item) => {
    return item.trim();
  });
  fileNameList.value = fileNameList.value.filter((item) => {
    return item.length > 0;
  });
}

listen<number>("update-progress", (event) => {
  updateProgress(event.payload);  // 忽略这个报错，这么写没问题
});

function updateProgress(percentage: number) {
  progressPercentage.value = percentage;
  if (percentage === 100) {
    progressStatus.value = "success";
  } else {
    progressStatus.value = "";
  }
}
</script>

<template>
  <div class="container">
    <!-- 头部操作区域 -->
    <div class="row gap-medium align-center header app-card">
      <div class="header-left">
        <el-text class="current-dir">当前目录：{{ currentDir || "无" }}</el-text>
        <el-button @click="openDir">打开</el-button>
        <el-radio-group v-model="recursiveRadio">
          <el-radio value="unrecursive">不递归</el-radio>
          <el-radio value="recursive">递归</el-radio>
        </el-radio-group>
      </div>
      <div class="header-right">
        <!-- 设置按钮 -->
        <el-button
          class="setting-btn"
          :icon="Setting"
          circle
          @click="openSettings"
        />
        <!-- 暗黑模式切换 -->
        <el-button 
          class="theme-toggle" 
          :icon="isDarkMode ? Sunny : Moon" 
          circle 
          @click="toggleDarkMode"
        />
        <div class="app-logo">
          <span class="logo-text">Pianer</span>
          <el-icon class="logo-icon"><Files /></el-icon>
        </div>
      </div>
      <!--      TODO：加一个筛选文件后缀的功能-->
    </div>

    <!-- 主体内容区域 -->
    <div class="row main-content">
      <!-- 左侧输入区域 -->
      <div class="input-wrapper column app-card">
        <div class="input-header">
          <el-text class="section-title">关键词列表</el-text>
        </div>
        <textarea
            v-model="fileNames"
            class="full-height-textarea"
            placeholder="请输入关键词，每行一个"
            @input="getFileNames">
        </textarea>
        <el-text class="file-count">目前关键词条数：{{ fileNameList.length }}</el-text>
      </div>

      <!-- 右侧操作区域 -->
      <OperationZone 
        :fileNameList="fileNameList" 
        :settings="settings" 
        class="operation-zone"
        ref="operationZoneRef"
      />
    </div>

    <div class="progress-info app-card">
      <el-progress :percentage="progressPercentage" :status="progressStatus"/>
    </div>

    <!-- 设置弹窗 -->
    <SettingsDialog 
      v-model:visible="settingsDialogVisible"
      :settings="settings"
      @save="handleSaveSettings"
    />
  </div>
</template>

<style scoped>
.row {
  display: flex;
  flex-direction: row;
}

.column {
  display: flex;
  flex-direction: column;
}

.gap-medium {
  gap: 16px;
}

.align-center {
  align-items: center;
}

.container {
  display: flex;
  flex-direction: column;
  height: 97vh;
  padding: 16px;
  box-sizing: border-box;
  background-color: var(--app-background-color);
  transition: background-color 0.3s ease;
}

.header {
  height: auto;
  flex-shrink: 0;
  margin-bottom: 16px;
  display: flex;
  justify-content: space-between;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 16px;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 16px;
}

.theme-toggle {
  transition: transform 0.3s ease;
}

.theme-toggle:hover {
  transform: rotate(30deg);
}

.app-logo {
  display: flex;
  align-items: center;
  gap: 6px;
  font-weight: bold;
  color: var(--el-color-primary);
}

.logo-text {
  font-size: 20px;
}

.logo-icon {
  font-size: 24px;
}

.current-dir {
  width: 300px;
}

.main-content {
  flex: 1;
  display: flex;
  gap: 20px;
  margin-top: 16px;
  margin-bottom: 16px;
  min-height: 0;
  /* 确保高度适当，给进度条留出空间 */
  height: calc(100% - 150px);
}

/* 左侧输入区域 */
.input-wrapper {
  width: calc(100% / 3); /* 宽度为1/3 */
  position: relative;
  padding-bottom: 40px;
  height: 100%;
  box-sizing: border-box;
  display: flex;
  flex-direction: column;
}

.input-header {
  margin-bottom: 12px;
}

.section-title {
  font-size: 16px;
  font-weight: bold;
}

.full-height-textarea {
  flex: 1;
  overflow-y: auto;
  overflow-x: auto;
  white-space: pre-wrap;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  padding: 8px;
  background-color: var(--card-background-color);
  color: var(--el-text-color-primary);
  transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
  resize: none;
}

/* 黑暗模式下的文本输入框焦点样式 */
html.dark .full-height-textarea:focus {
  outline: none;
  border-color: var(--el-color-primary);
  box-shadow: 0 0 0 2px rgba(64, 158, 255, 0.2);
}

.file-count {
  position: absolute;
  left: 10px;
  bottom: 10px;
  z-index: 2;
  padding: 4px 10px;
  border-radius: 4px;
  background-color: var(--el-border-color-lighter);
  color: var(--el-text-color-primary);
  transition: background-color 0.3s ease, color 0.3s ease;
}

/* 自定义radio样式 */
:deep(.el-radio__input.is-checked .el-radio__inner) {
  border-color: var(--el-color-primary);
  background: var(--el-color-primary);
}

:deep(.el-radio__input.is-checked + .el-radio__label) {
  color: var(--el-text-color-primary);
}

:deep(.el-radio__label) {
  color: var(--el-text-color-regular);
  transition: color 0.3s ease;
}

:deep(.el-radio__inner) {
  background-color: var(--el-fill-color-blank);
  border-color: var(--el-border-color);
  transition: background-color 0.3s ease, border-color 0.3s ease;
}

/* 进度条样式调整 */
:deep(.el-progress-bar__outer) {
  background-color: var(--el-border-color-lighter);
}

:deep(.el-progress__text) {
  color: var(--el-text-color-primary);
}

.progress-info {
  margin-bottom: 0;
  width: auto;
}

.operation-zone {
  width: calc(100% / 3 * 2); /* 宽度为2/3 */
  height: 100%;
  box-sizing: border-box;
  display: flex;
}

.setting-btn {
  transition: transform 0.3s ease;
}

.setting-btn:hover {
  transform: rotate(30deg);
}
</style>