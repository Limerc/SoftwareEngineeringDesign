import { req } from "./req"

const noAuth = true

export interface LoginReq {
  username: string
  password: string
}

export interface RegisterReq {
  username: string
  password: string
}

export const register = (data: RegisterReq): Promise<{ data: string } | string> =>
  req({ method: 'POST', url: '/auth/register', data, noAuth })


/**
 *
 * @param data
 * @returns jwt token
 */
export const login = (data: LoginReq): Promise<{ data: string } | string> =>
  req({ method: 'POST', url: '/auth/login', data, noAuth })


// export interface UserInfo {
//   id: number
//   username: string
//   password: string
//   phone: string
//   ID_number: string
//   Is_admin: number
// }

// export const getUserInfo = async (explicit = true): Promise<UserInfo> => {
//   if (!getStore().getToken())
//     return Promise.reject('登录超时')
//   const res = await req({ url: '/user/userInfo', showError: explicit, showLoading: false })
//   return { ...res, Is_admin: res.isAdmin, ID_number: res.IDNumber, id: res.u_id }
// }

// export type UpdateUserInfoForm = Pick<UserInfo, 'username' | 'phone' | 'ID_number'>

// export const updateUserInfo = (data: UpdateUserInfoForm & { uId: number }): Promise<void> =>
//   req({ method: 'PUT', url: '/user/update', data: { ...data, IDNumber: data.ID_number, u_id: data.uId }, contentType: ContentType.JSON })

// export interface RePasswordForm {
//   old_pwd: string
//   new_pwd: string
//   re_pwd: string
// }

// export const rePassword = (data: RePasswordForm): Promise<void> =>
//   req({ method: 'PATCH', url: '/user/updatePwd', data, contentType: ContentType.JSON })
