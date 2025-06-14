<script setup lang="ts">
  import { getProblems, type Problem } from '@/apis/problems';
  import { useProblemStore } from '@/stores/problemStore';
  import { ElCol, ElOption, ElRow, ElSelect, ElTag } from 'element-plus';
  import { ref } from 'vue';
  import { useRouter } from 'vue-router';

  // const problemStore = useProblemStore()
  const problems = ref<Problem[]>([])
  // const pageNum = ref(0)
  // const pageSize = ref(10)
  // const difficulty = ref<"easy" | "medium" | "hard">()
  const router = useRouter()

  // const difficultyNameMap = {
  //   easy: '简单',
  //   medium: '中等',
  //   hard: '困难',
  // }

  // const difficultyTypeMap: {
  //   [key: string]: 'success' | 'warning' | 'danger'
  // } = {
  //   easy: 'success',
  //   medium: 'warning',
  //   hard: 'danger',
  // }

  // let tot = 0
  async function getList() {
    const res = await getProblems()
    problems.value = res
  }

  function goDetail(id: number) {
    // problemStore.setProblem(problems.value[idx])
    router.push({ name: 'pratice', query: { id } })
  }

  getList()
</script>

<template>
  <div class="container">
    <div class="mid">
      <!-- <ElSelect clearable @change="getList" v-model="difficulty" style="width: 200px;" placeholder="请选择题目难度">
        <ElOption label="简单" value="easy" />
        <ElOption label="中等" value="medium" />
        <ElOption label="困难" value="hard" />
      </ElSelect> -->
      <div class="problem-list">
        <ElRow style="display: flex;
          align-items: center;
          outline: 1px solid #eee;">
          <ElCol class="table-head" :span="6">题目</ElCol>
          <ElCol class="table-head" :span="6">难度</ElCol>
          <ElCol class="table-head" :span="6">通过率</ElCol>
          <ElCol class="table-head" :span="6">提交次数</ElCol>
        </ElRow>
        <ElRow @click="goDetail(p.id)" v-for="(p) in problems" :key="p.id" class="problem">
          <ElCol :span="6">
            <div class="title">
              <div class="name">
                {{ p.title }}
              </div>
              <div class="tags">
                <!-- <ElTag v-for="t in p.tags" :key="t" effect="dark">{{ t }}</ElTag> -->
              </div>
            </div>
          </ElCol>
          <ElCol :span="6">
            <!-- <ElTag effect="dark" :type="difficultyTypeMap[p.difficulty]">{{ difficultyNameMap[p.difficulty] }}</ElTag> -->
            -
          </ElCol>
          <ElCol :span="6">
            <div class="accept-rate">
              0
            </div>
          </ElCol>
          <ElCol :span="6">
            <div class="submit-count">
              0
            </div>
          </ElCol>
        </ElRow>
        <div>

        </div>
      </div>
    </div>
  </div>
</template>

<style lang="scss" scoped>
  .container {
    // 原display: flex; justify-content: center;
    display: block;
    /* 改用块级布局 */
    padding: 2rem 0;

    .mid {
      width: 90%;
      /* 相对父容器宽度 */
      max-width: 1200px;
      /* 最大宽度限制 */
      margin: 0 auto;
      /* 关键：左右margin自动分配实现居中 */
      box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
      border-radius: 12px;
      overflow: hidden;
    }
  }

  .problem-list {
    .table-head {
      padding: 1.2rem;
      background: linear-gradient(45deg, #6b48ff, #7851ff);
      color: white;
      font-size: 1.1rem;
      border-radius: 8px 8px 0 0;
    }

    .problem {
      padding: 1.2rem;
      // 原margin: 10px 0;
      margin: 10px auto;
      /* 新增auto实现水平居中 */
      background: white;
      border-radius: 8px;
      transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);

      &:hover {
        transform: translateY(-2px);
        box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
        background: #f8f9ff;
      }

      .title {
        .name {
          font-size: 1.1rem;
          font-weight: 600;
          color: #2d3748;
        }

        .tags {
          margin-top: 0.5rem;
        }
      }

      .accept-rate {
        font-weight: 500;
        color: #48bb78;
      }

      .submit-count {
        color: #718096;
      }
    }
  }


  .el-tag {
    border-radius: 4px;
    padding: 0 0.5rem;
    height: 24px;
    line-height: 24px;
  }
</style>
