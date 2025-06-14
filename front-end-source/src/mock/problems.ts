import { mock } from "mockjs";

mock(/^\/api\/problems/, 'get', {
  "code": 0,
  "message": "操作成功",
  "total": 100,
  "problems": [
    {
      "id": 1,
      "title": "所有权练习",
      "description": "修复以下代码...",
      "tags": ["ownership", "basic"],
      "difficulty": "easy",
      "submit_count": 120,
      "accept_rate": 0.75
    },
    {
      "id": 2,
      "title": "所有权练习2",
      "description": "修复以下代码...",
      "tags": ["ownership", "basic"],
      "difficulty": "easy",
      "submit_count": 120,
      "accept_rate": 0.75
    },
    {
      "id": 3,
      "title": "所有权练习3",
      "description": "修复以下代码...",
      "tags": ["ownership", "basic"],
      "difficulty": "easy",
      "submit_count": 120,
      "accept_rate": 0.75
    },
  ]
})

// 新增的提交题目mock
mock(/^\/api\/problems\/\d+\/submit/, 'post', () => {
  // 随机生成测试用例数量(3-8个)
  const total = Math.floor(Math.random() * 6) + 3;
  // 随机生成通过的用例数量(0-total)
  const passed = Math.floor(Math.random() * (total + 1));

  // 生成测试用例详情
  const details = Array.from({ length: total }, (_, i) => ({
    case_id: i + 1,
    status: i < passed ? 'passed' : 'failed'
  }));

  return {
    code: 0,
    message: "提交成功",
    passed,
    total,
    details
  };
});
