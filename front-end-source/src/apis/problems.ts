import { req } from './req'

export interface Problem {
  id: number;
  title: string;
  description: string;
  examples: string;
  // tags: string[];
  // difficulty: 'easy' | 'medium' | 'hard';
  // submit_count: number;
  // accept_rate: number;
}

interface GetProblemResp {
  code: number;
  message: string;
  total: number;
  problems: Problem[];
}

export const getProblems = (): Promise<Problem[]> => req('/problems')

export const getProblemDetail = (id: number): Promise<Problem> => req(`/problems/${id}`)

// interface SubmitProblemResp {
//   code: number;
//   message: string;
//   passed: number;
//   total: number;
//   details: Detail[];
// }
interface SubmitProblemResp {
  success: boolean;
  message: string;
  stdout: string;
  stderr: string;
}

interface Detail {
  case_id: number;
  status: 'passed' | 'failed';
}
export const submitProblem = (id: number, data: { code: string }): Promise<SubmitProblemResp> =>
  req({ url: `/judge`, method: 'POST', data: { ...data, problem_id: id } })
