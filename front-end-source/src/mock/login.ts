import { mock } from "mockjs";

mock('/api/user/register', 'post', {
  "message": "",
  "code": 200,
  "data": "注册成功"
})

mock('/api/user/loginIn', 'post', {
  "message": "",
  "code": 200,
  "data": `eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJjbGFpbXMiOnsiaWQiOjUsInVzZXJuYW1lIjoid2FuZ2JhIn0sImV4cCI6MTY5MzcxNTk3OH0.pE_RATcoF7Nm9KEp9eC3CzcBbKWAFOL0IsuMNjnZ95M`
})
