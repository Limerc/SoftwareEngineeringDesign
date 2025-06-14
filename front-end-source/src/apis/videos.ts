import { req } from "./req";

export const getVideos = (kw: string): Promise<{ code: any, message: any, data: { title: string, url: string }[] }> =>
  req(`/video/search_videos?keyword=${kw}`)
