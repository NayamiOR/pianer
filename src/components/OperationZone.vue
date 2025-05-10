<!--
  - Copyright 2025. NayamiOR
  -
  -    Licensed under the Apache License, Version 2.0 (the "License");
  -    you may not use this file except in compliance with the License.
  -    You may obtain a copy of the License at
  -
  -        http://www.apache.org/licenses/LICENSE-2.0
  -
  -    Unless required by applicable law or agreed to in writing, software
  -    distributed under the License is distributed on an "AS IS" BASIS,
  -    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  -    See the License for the specific language governing permissions and
  -    limitations under the License.
  -->

<script lang="ts" setup>
import {ref, Ref} from "vue";
import {message, open} from "@tauri-apps/plugin-dialog";
import {invoke} from "@tauri-apps/api/core";

const props = defineProps(['fileNameList', 'settings'])
const method: Ref<string> = ref("same");
const operation: Ref<string> = ref("copy");
const targetDir: Ref<string | null> = ref(null);
const canExecute: Ref<boolean> = ref(false);
const fileListData: Ref<{ file_path: string, file_name: string, keyword: string }[]> = ref([]);

async function setTargetDir() {
  const path = await open({
    directory: true,
    multiple: false,
    title: "选择目录",
  });

  targetDir.value = path;

  try {
    await invoke("set_target_folder", {targetPath: path});
  } catch (e) {
    await message('选择目录失败，原因：' + e);
  }
}

async function openTargetDir() {
  try {
    const result = await invoke('open_target_folder');
    console.log(result);
  } catch (err) {
    await message('打开目录失败，原因：' + err);
  }

}

async function search() {
  try {
    const result = await invoke("search", {
      method: method.value,
      keywords: props.fileNameList,
    });
    fileListData.value = result as { file_path: string, file_name: string, keyword: string }[];
    canExecute.value = fileListData.value.length > 0;
    console.log(result);
  } catch (err) {
    await message('搜索失败，原因：' + err);
  }
}

async function execute() {
  try {
    const result = await invoke("execute", {
      method: operation.value,
    });

    console.log(result);
  } catch (err) {
    await message('执行失败，原因：' + err);
  }
}
</script>

<template>
  <div class="column operations">
    <div class="column gap-small operation-panel app-card">
      <div class="panel-header">
        <el-text class="section-title">操作配置</el-text>
        <div class="row gap-small align-center target-dir">
          <el-text>存放目录：{{ targetDir || "无，请选择" }}</el-text>
          <el-button size="small" @click="setTargetDir">选择</el-button>
          <el-button size="small" @click="openTargetDir">打开</el-button>
        </div>
      </div>

      <!-- 操作和查找方式区域 - 四列网格布局 -->
      <div class="section-divider"></div>
      <div class="option-grid">
        <!-- 操作区域 - 占第一列 -->
        <div class="column gap-small option-block">
          <el-text class="section-title">操作：</el-text>
          <el-radio-group v-model="operation" class="radio-group">
            <el-radio value="copy">复制</el-radio>
            <el-radio value="move">移动</el-radio>
            <el-radio value="delete">删除</el-radio>
          </el-radio-group>
        </div>

        <!-- 查找方式区域 - 占第二列 -->
        <div class="column gap-small option-block">
          <el-text class="section-title">查找方式：</el-text>
          <el-radio-group v-model="method" class="radio-group">
            <el-radio value="same">同名且格式相同</el-radio>
            <el-radio value="same-name">同文件名</el-radio>
            <el-radio value="include-name">包含文件名</el-radio>
          </el-radio-group>
        </div>
        
        <!-- 第三列和第四列留空 -->
        <div class="option-empty"></div>
        <div class="option-empty"></div>
      </div>
      
      <div class="action-buttons">
        <el-button type="primary" @click="search">搜索</el-button>
        <el-button type="success" :disabled="!canExecute" @click="execute">执行</el-button>
      </div>
    </div>

    <div class="file-list-container app-card">
      <div class="list-header">
        <el-text class="section-title">搜索结果</el-text>
        <el-text class="result-count" v-if="fileListData.length > 0">共 {{ fileListData.length }} 个文件</el-text>
      </div>
      <el-scrollbar class="table-scrollbar">
        <el-table
            :data="fileListData"
            empty-text="暂无数据"
            style="width: 100%"
        >
          <el-table-column label="序号" type="index" width="60"/>
          <el-table-column label="文件名" prop="file_name"/>
          <el-table-column label="匹配词" prop="keyword"/>
          <el-table-column label="文件路径" prop="file_path" show-overflow-tooltip/>
        </el-table>
      </el-scrollbar>
    </div>
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

/* 间距控制 */
.gap-small {
  gap: 8px;
}

.gap-medium {
  gap: 16px;
}

.align-center {
  align-items: center;
}

.operations {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  gap: 16px;
  box-sizing: border-box;
}

.operation-panel {
  flex-shrink: 0;
  transition: background-color 0.3s ease, color 0.3s ease;
  max-height: 38%; /* 稍微减小高度，由于布局优化后需要的垂直空间减少了 */
  overflow: auto;
}

.panel-header, .list-header {
  margin-bottom: 8px; /* 减小标题底部边距 */
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.target-dir {
  display: flex;
  align-items: center;
  gap: 8px;
  font-size: 14px;
}

.target-dir .el-button {
  padding: 6px 12px;
  font-size: 12px;
}

.section-title {
  font-size: 15px; /* 稍微减小标题字体大小 */
  font-weight: bold;
}

.result-count {
  color: var(--el-text-color-secondary);
  font-size: 14px;
}

.section-divider {
  height: 1px;
  background: var(--el-border-color-lighter);
  margin: 6px 0; /* 减小上下间距 */
  transition: background-color 0.3s ease;
}

.action-buttons {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
  margin-top: 10px;
}

:deep(.el-button) {
  padding: 8px 15px; /* 稍微减小按钮内边距 */
}

/* 文件列表区域 */
.file-list-container {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  transition: background-color 0.3s ease, color 0.3s ease;
  min-height: 0;
  /* 调整内边距，使内容区域更加紧凑 */
  padding: 12px var(--panel-padding);
}

.list-header {
  flex-shrink: 0;
}

.table-scrollbar {
  flex: 1;
  overflow: auto;
  margin-top: 4px; /* 添加一点顶部边距 */
}

.el-scrollbar {
  height: calc(100% - 40px); /* 减去header的高度 */
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

/* 表格样式增强 */
:deep(.el-table) {
  background-color: transparent;
  transition: background-color 0.3s ease, color 0.3s ease;
  font-size: 14px; /* 稍微减小表格字体大小 */
}

:deep(.el-table th),
:deep(.el-table tr) {
  background-color: transparent;
  transition: background-color 0.3s ease, color 0.3s ease;
}

:deep(.el-table td) {
  background-color: transparent;
  border-bottom-color: var(--el-border-color-lighter);
  transition: background-color 0.3s ease, color 0.3s ease, border-color 0.3s ease;
}

:deep(.el-table th.is-leaf) {
  background-color: var(--el-fill-color-light);
  border-bottom-color: var(--el-border-color);
}

:deep(.el-table--border) {
  border-color: var(--el-border-color);
}

:deep(.el-table__empty-text) {
  color: var(--el-text-color-secondary);
}

:deep(.el-table__header-wrapper) {
  border-bottom: 1px solid var(--el-border-color);
}

:deep(.el-table--enable-row-hover .el-table__body tr:hover > td) {
  background-color: var(--el-fill-color-light);
}

:deep(.el-table__row) {
  color: var(--el-text-color-regular);
}

:deep(.el-table__header) {
  color: var(--el-text-color-primary);
}

/* 滚动条样式 */
:deep(.el-scrollbar__bar) {
  background-color: var(--el-border-color-lighter);
}

:deep(.el-scrollbar__thumb) {
  background-color: var(--el-text-color-secondary);
}

/* 操作和查找方式区域的网格布局 */
.option-grid {
  display: grid;
  grid-template-columns: repeat(4, 1fr); /* 将行分为4等份 */
  gap: 16px;
  width: 100%;
}

.option-block {
  border: 1px solid var(--el-border-color-lighter);
  border-radius: 4px;
  padding: 8px 10px;
  background-color: var(--card-background-color);
  transition: background-color 0.3s ease, border-color 0.3s ease;
}

.option-empty {
  /* 占位元素，可以根据需要设置样式 */
}

/* 深色模式下方块边框颜色 */
html.dark .option-block {
  border-color: var(--el-border-color);
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 4px; /* 控制单选按钮之间的间距 */
}

:deep(.el-radio) {
  margin-right: 0;
  margin-bottom: 0; /* 移除底部边距，使用gap控制间距 */
  height: 28px; /* 减小单选按钮高度 */
  line-height: 28px;
}
</style>