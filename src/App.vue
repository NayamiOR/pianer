<script lang="ts" setup>
import {Ref, ref} from "vue";
import {message, open} from "@tauri-apps/plugin-dialog";
import OperationZone from "./components/OperationZone.vue";
import {invoke} from "@tauri-apps/api/core";
import {listen} from "@tauri-apps/api/event";

const currentDir: Ref<string | null> = ref(null);
const recursiveRadio: Ref<string> = ref("unrecursive");
const fileNames: Ref<string | null> = ref(null);
const fileNameList: Ref<string[]> = ref([]);

const progressPercentage: Ref<number> = ref(0);
const progressStatus: Ref<string> = ref("");

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
    <div class="row gap-medium align-center header">
      <el-text class="current-dir">当前目录：{{ currentDir || "无" }}</el-text>
      <el-button @click="openDir">打开</el-button>
      <el-radio-group v-model="recursiveRadio">
        <el-radio value="unrecursive">不递归</el-radio>
        <el-radio value="recursive">递归</el-radio>
      </el-radio-group>
      <!--      TODO：加一个筛选文件后缀的功能-->
    </div>

    <!-- 主体内容区域 -->
    <div class="row main-content">
      <!-- 左侧输入区域 -->
      <div class="input-wrapper column">
        <textarea
            v-model="fileNames"
            class="full-height-textarea"
            placeholder="请输入内容"
            @input="getFileNames">
        </textarea>
        <el-text class="file-count">目前关键词条数：{{ fileNameList.length }}</el-text>
      </div>

      <!-- 右侧操作区域 -->
      <OperationZone class="operation-zone" :fileNameList="fileNameList"/>
    </div>

    <div class="progress-info">
      <el-progress :percentage="progressPercentage" :status="progressStatus"/>
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
}

.header {
  height: 50px;
  flex-shrink: 0;
}

.current-dir {
  width: 300px;
}

.main-content {
  flex: 1;
  gap: 20px;
  margin-top: 16px;
  min-height: 0;
}

/* 左侧输入区域 */
.input-wrapper {
  flex: 1;
  position: relative;
  min-height: 0;
}

.full-height-textarea {
  height: 90%;
  overflow-x: auto;
  white-space: nowrap;
  font-family: Microsoft YaHei;
}

.file-count {
  position: absolute;
  left: 10px;
  bottom: 10px;
  z-index: 2;
  background: rgba(255, 255, 255, 0.8);
  padding: 2px 8px;
  border-radius: 4px;
}

.progress-info {
  margin-top: 16px;
  width: 100%;
}

.operation-zone {
  flex: 2;
  margin-right: 0px;
}

</style>