import { req } from "./req";


export interface BlogPost {
  id: number;
  title: string;
  content: string;
  related_problem?: string;
  created_at?: Date;
  comment_count?: number;
}

export interface BlogListResponse {
  code: number;
  message?: string;
  posts: BlogPost[];
}


export const getBlogs = (params: { problem: string, page: number, per_page: number }): Promise<BlogListResponse> =>
  req({ url: '/blog/get_blog_list', params })


// related_problem将题目id转换为字符串传入
export const addBlog = (data: { title: string, content: string, related_problem: string }) =>
  req({ url: '/blog/add_blog', method: 'POST', data })


export interface Comment {
  id: number;
  blog_id: number;
  content: string;
  created_at?: Date;
}

export interface CommentListResponse {
  comments: Comment[];
}


export const addComment = (blogId: number, data: { content: string }) =>
  req({ url: `/blog/add_comment/${blogId}`, method: 'POST', data })


export const getComments = (blogId: number): Promise<CommentListResponse> =>
  req({ url: `/blog/get_comments/${blogId}` })
