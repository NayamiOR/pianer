<script lang="ts" setup>
import {ref, Ref} from "vue";
import {open} from "@tauri-apps/plugin-dialog";

const method: Ref<string> = ref("same");
const operation: Ref<string> = ref("copy");
const targetDir: Ref<string | null> = ref(null);
const canExecute: Ref<boolean> = ref(false);
const fileListData: Ref<any[]> = ref([
  {name: 'file1.txt', path: '/documents', size: '1.2MB'},
  {name: 'image.png', path: '/pictures', size: '3.4MB'},
  {name: 'report.pdf', path: '/documents', size: '2.1MB'},
]);

async function setTargetDir() {
  const dir = await open(
      {
        directory: true,
        multiple: false,
        title: "选择目录",
      }
  )
  if (dir) {
    targetDir.value = dir;
  }
}

function search() {

}

function execute() {

}
</script>

<template>
  <div class="column gap-medium operations">
    <div class="column gap-small operation-panel">
      <div class="row gap-small align-center">
        <el-text>存放目录：{{ targetDir || "无，请选择" }}</el-text>
        <el-button @click="setTargetDir">打开</el-button>
      </div>

      <!-- 操作区域 -->
      <div class="section-divider"></div>
      <div class="column gap-small">
        <el-text class="section-title">操作：</el-text>
        <el-radio-group v-model="operation">
          <el-radio value="copy">复制</el-radio>
          <el-radio value="move">移动</el-radio>
          <el-radio value="delete">删除</el-radio>
        </el-radio-group>
      </div>

      <!-- 查找方式区域 -->
      <div class="section-divider"></div>
      <div class="column gap-small">
        <el-text class="section-title">查找方式：</el-text>
        <el-radio-group v-model="method">
          <el-radio value="same">同名且格式相同</el-radio>
          <el-radio value="same-name">同文件名</el-radio>
          <el-radio value="include-name">包含文件名</el-radio>
        </el-radio-group>
      </div>
    </div>

    <div class="row gap-small action-buttons">
      <el-button @click="search">搜索</el-button>
      <el-button :disabled="!canExecute" @click="execute">执行</el-button>
    </div>


    <div class="file-list-container">
      <el-scrollbar>
        <el-table
            :data="fileListData"
            empty-text="暂无数据"
            height="100%"
            style="width: 100%"
        >
          <el-table-column label="No." prop="index" resizable width="50px"/>
          <el-table-column label="文件名" prop="name"/>
          <el-table-column label="匹配词" prop="keyword"/>
          <el-table-column label="状态" prop="status"/>

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
  width: 400px;
  display: flex;
  flex-direction: column;
  min-height: 0;
  margin-right: 16px;
}

.operation-panel {
  flex-shrink: 0;
}

.section-title {
  font-weight: bold;
  margin-bottom: 4px;
}

.section-divider {
  height: 1px;
  background: var(--el-border-color);
  margin: 12px 0;
}

.action-buttons {
  margin: 16px 0;
  flex-shrink: 0;
}

/* 文件列表区域 */
.file-list-container {
  flex: 1;
  min-height: 100px;
  height: 200px;
  max-height: 300px;
  border: 1px solid var(--el-border-color);
  border-radius: 4px;
  overflow: hidden;
}
</style>