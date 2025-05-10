<template>
  <el-dialog
      v-model="dialogVisible"
      title="设置"
      width="30%"
      center
      class="settings-dialog"
  >
    <div class="settings-content">
      <el-form label-position="top">
        <el-form-item label="文件扩展名过滤（多个用逗号分隔，如：jpg,png）">
          <el-input v-model="settingsData.fileExtensions" placeholder="例如：jpg,png,mp4"></el-input>
        </el-form-item>
        <el-form-item label="最大文件大小 (MB)">
          <el-input-number v-model="settingsData.maxFileSize" :min="1" :max="1000"></el-input-number>
        </el-form-item>
        <el-form-item>
          <el-checkbox v-model="settingsData.skipHiddenFiles">跳过隐藏文件</el-checkbox>
        </el-form-item>
      </el-form>
    </div>
    <template #footer>
        <span class="dialog-footer">
          <el-button @click="cancel">取消</el-button>
          <el-button type="primary" @click="save">保存</el-button>
        </span>
    </template>
  </el-dialog>
</template>

<script lang="ts" setup>
import { ref, computed, defineProps, defineEmits, watch } from 'vue';
import { message } from "@tauri-apps/plugin-dialog";

const props = defineProps({
  visible: {
    type: Boolean,
    default: false
  },
  settings: {
    type: Object,
    default: () => ({
      fileExtensions: '',
      maxFileSize: 100,
      skipHiddenFiles: true
    })
  }
});

const emit = defineEmits(['update:visible', 'save']);

// 对话框显示状态，与父组件双向绑定
const dialogVisible = computed({
  get: () => props.visible,
  set: (value) => emit('update:visible', value)
});

// 复制一份设置数据，避免直接修改props
const settingsData = ref({...props.settings});

// 监听props变化更新本地数据
watch(() => props.settings, (newSettings) => {
  settingsData.value = {...newSettings};
}, { deep: true });

// 取消操作
function cancel() {
  dialogVisible.value = false;
}

// 保存设置
function save() {
  emit('save', settingsData.value);
  dialogVisible.value = false;
  message("设置已保存");
}
</script>

<style scoped>
/* 设置对话框样式 */
.settings-content {
  margin: 20px 0;
}

.dialog-footer {
  display: flex;
  justify-content: flex-end;
  gap: 10px;
}

/* 自定义对话框样式 */
:deep(.el-dialog) {
  border-radius: var(--card-border-radius);
  box-shadow: var(--card-shadow);
  background-color: var(--card-background-color);
  transition: background-color 0.3s ease;
}

:deep(.el-dialog__header) {
  border-bottom: 1px solid var(--el-border-color-lighter);
  padding-bottom: 15px;
  margin-bottom: 0;
}

:deep(.el-dialog__title) {
  font-weight: bold;
  color: var(--el-text-color-primary);
}

:deep(.el-dialog__body) {
  color: var(--el-text-color-regular);
}

:deep(.el-form-item__label) {
  font-weight: 500;
  color: var(--el-text-color-primary);
}

:deep(.el-input__wrapper),
:deep(.el-input-number__wrapper) {
  background-color: var(--el-fill-color-blank);
  box-shadow: 0 0 0 1px var(--el-border-color) inset;
}

:deep(.el-input__inner),
:deep(.el-input-number__decrease),
:deep(.el-input-number__increase) {
  background-color: transparent;
  color: var(--el-text-color-primary);
}

:deep(.el-input-number__decrease),
:deep(.el-input-number__increase) {
  border-color: var(--el-border-color);
}

:deep(.el-checkbox__inner) {
  background-color: var(--el-fill-color-blank);
  border-color: var(--el-border-color);
}

:deep(.el-checkbox__input.is-checked .el-checkbox__inner) {
  background-color: var(--el-color-primary);
  border-color: var(--el-color-primary);
}

:deep(.el-checkbox__label) {
  color: var(--el-text-color-primary);
  transition: color 0.3s ease;
}

:deep(.el-dialog__footer) {
  border-top: 1px solid var(--el-border-color-lighter);
  padding-top: 15px;
}
</style>