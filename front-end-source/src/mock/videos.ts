import { mock } from "mockjs";

mock(/^\/api\/videos/, 'get', {
  "code": 0,
  "message": "操作成功",
  "data": [
    {
      "title": "动态规划",
      "url": "https://billbill/dp_learning"
    },
    {
      "title": "递归",
      "url": "https://billbill/digui_learning"
    }
  ]
})
