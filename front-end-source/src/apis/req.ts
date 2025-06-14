import { useUserStore } from "@/stores/userStore"
import axios, { type AxiosInstance } from "axios"
import { ElLoading, ElMessage, ElMessageBox } from "element-plus"
import router from "@/router/"
// import qs from 'qs'
import { AUTH, ContentType, type Method, type RequestOptions } from "."

let loading: ReturnType<typeof ElLoading.service> | null = null

let ins: AxiosInstance | null = null
function getIns() {
  if (ins)
    return ins
  ins = axios.create({
    baseURL: import.meta.env.PROD ? `/api1` : `/api1`,
    timeout: 10000,
    // paramsSerializer: params =>
    //   qs.stringify(params, {
    //     arrayFormat: 'repeat',
    //     skipNulls: true,
    //   })
  })

  //
  ins.interceptors.request.use(
    config => {
      if (!loading && (config as any).showLoading) {
        loading = ElLoading.service({
          lock: true,
          text: '加载中',
          background: 'rgba(0, 0, 0, .3)',
        })
      }
      return config
    },
    err => {
      if (loading) {
        loading.close()
        loading = null
      }
      ElMessage.error('请求失败')
      return Promise.reject(err)
    })
  //
  ins.interceptors.response.use(
    resp => {
      if (loading) {
        loading.close()
        loading = null
      }
      const { data } = resp
      const { responseType } = resp.config
      if (responseType == "arraybuffer" || responseType == "blob") {
        return data
      }
      if (!data.code)
        return data
      if ((resp.config as any).showError) {
        ElMessage.error(data.errorMessage)
      }
      return Promise.reject(data)
    }, err => {
      if (loading) {
        loading.close()
        loading = null
      }
      if (err.status === 401) {
        useUserStore().clearToken()
        useUserStore().clearUser()
        ElMessageBox.alert('登录过期，请重新登录', '提示', {
          confirmButtonText: '去登陆',
          type: 'warning',
          showClose: false
        }).then(() => {
          router.push({ name: 'login' })
        })
        return Promise.reject(err)
      }
      ElMessage.error('请求失败')
      return Promise.reject(err)
    }
  )
  return ins
}

export function req(url: string, method?: Method): Promise<any>
export function req(config: RequestOptions): Promise<any>
export async function req(configOrUrl: string | RequestOptions, method: Method = 'GET'): Promise<any> {
  let config: RequestOptions = {
    url: '',
    method,
    showLoading: true,
    showError: true
  }
  if (typeof configOrUrl === 'string')
    config = { ...config, url: configOrUrl }
  else
    config = { ...config, ...configOrUrl }
  // 处理content-type
  if (config.method !== 'GET')
    config.contentType = config.contentType ?? ContentType.JSON
  if (config.contentType === ContentType.FORM) {
    const formData = new FormData()
    for (const key in config.data) {
      formData.append(key, config.data[key] ?? '')
    }
    config.data = formData
  }
  // else if (config.contentType === ContentType.URL) {
  //   const params = new URLSearchParams()
  //   for (const key in config.data) {
  //     params.append(key, config.data[key] ?? '')
  //   }
  //   config.params = params
  // }
  // 处理请求头
  const headers: any = {
    'Content-Type': config.contentType,
    'X-Requested-With': 'XMLHttpRequest'
  }
  if (!config.noAuth)
    headers[AUTH] = 'Bearer ' + useUserStore().getToken()
  const resConfig = { ...config, headers }
  return getIns()(resConfig)
}
