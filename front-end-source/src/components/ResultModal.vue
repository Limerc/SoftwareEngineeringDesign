<template>
  <el-dialog :model-value="visible" @update:model-value="emit('update:visible', $event)" title="提交结果" width="700px"
    :before-close="handleClose" center>
    <div class="result-content">
      <div class="summary">
        <div class="status-text" :class="statusClass">
          <el-icon :size="24">
            <component :is="statusIcon" />
          </el-icon>
          <span>{{ statusText }}</span>
        </div>
      </div>
      <el-card :class="['case-item']" :body-style="{ padding: '10px' }">
        <div class="case-header">
          <el-tag>
            {{ '最后一个样例' }}
          </el-tag>
        </div>
        <div class="case-details">
          <p class="detail-line">
            <span class="detail-value">{{ message }}</span>
          </p>
        </div>
      </el-card>
      <div class="case-list">
        <el-card :class="['case-item', 'success']" :body-style="{ padding: '10px' }">
          <div class="case-header">
            <el-tag :type="'success'">
              {{ '标准输出' }}
            </el-tag>
          </div>
          <div class="case-details">
            <p class="detail-line">
              <span class="detail-value">{{ stdout }}</span>
            </p>
          </div>
        </el-card>

        <el-card :class="['case-item', 'error']" :body-style="{ padding: '10px' }">
          <div class="case-header">
            <el-tag :type="'danger'">
              {{ '错误输出' }}
            </el-tag>
          </div>
          <div class="case-details">
            <p class="detail-line">
              <span class="detail-value">{{ stderr }}</span>
            </p>
          </div>
        </el-card>
      </div>
    </div>

    <template #footer>
      <div class="dialog-footer">
        <el-button type="primary" @click="handleClose">确定</el-button>
      </div>
    </template>
  </el-dialog>
</template>

<script setup lang="ts">
  import { computed } from 'vue';
  import { CircleCheck, CircleClose } from '@element-plus/icons-vue';
  import { ElDialog, ElCard, ElTag, ElButton, ElIcon } from 'element-plus';


  const props = defineProps<{
    visible: boolean;
    success: boolean;
    message: string;
    stdout?: string;
    stderr?: string;
  }>();

  const emit = defineEmits(['update:visible']);

  const handleClose = () => {
    emit('update:visible', false);
  };


  const statusClass = computed(() => {
    if (props.success) return 'success';
    return 'error';
  });

  const statusIcon = computed(() => {
    if (props.success) return CircleCheck;
    return CircleClose;
  });

  const statusText = computed(() => {
    if (props.success) return '全部通过！恭喜你！';
    return `未能全部通过，继续努力！`;
  });
</script>

<style scoped lang="scss">
  .result-content {
    padding: 10px 0;
    overflow-y: auto;
    height: 50vh;
  }

  .summary {
    margin-bottom: 20px;
    padding-bottom: 20px;
    border-bottom: 1px solid #ebeef5;
  }

  .stats {
    display: flex;
    justify-content: space-around;
    margin-bottom: 15px;

    .stat {
      text-align: center;

      .label {
        font-size: 14px;
        color: #606266;
        display: block;
      }

      .value {
        font-size: 24px;
        font-weight: 500;
        margin-top: 5px;
      }

      .success {
        color: #67c23a;
      }

      .error {
        color: #f56c6c;
      }
    }
  }

  .progress-bar {
    height: 8px;
    background-color: #e4e7ed;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 15px;
  }

  .progress-success {
    height: 100%;
    background-color: #67c23a;
  }

  .progress-fail {
    height: 100%;
    background-color: #f56c6c;
  }

  .status-text {
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 16px;
    font-weight: 500;

    .success {
      color: #67c23a;
    }

    .error {
      color: #f56c6c;
    }

    .partial {
      color: #e6a23c;
    }
  }

  .case-list {
    display: grid;
    grid-template-columns: 1fr;
    gap: 15px;
  }

  .case-item {
    border-radius: 6px;

    .case-header {
      display: flex;
      justify-content: space-between;
      align-items: center;
      margin-bottom: 8px;
    }

    .case-id {
      font-weight: 500;
      color: #303133;
    }

    .detail-line {
      margin-top: 8px;
      display: flex;

      .detail-label {
        min-width: 80px;
        color: #909399;
      }

      .detail-value {
        color: #606266;
      }
    }

    &.success {
      border-left: 4px solid #67c23a;
    }

    &.error {
      border-left: 4px solid #f56c6c;
    }
  }
</style>
