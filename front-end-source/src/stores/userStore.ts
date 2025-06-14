import { defineStore } from "pinia";
import { ref } from "vue";

export interface User {
  username: string;
}
const USER_INFO_KEY = 'USER_INFO_KEY'
const TOKEN_KEY = 'TOKEN_KEY'

export const useUserStore = defineStore('user', () => {
  const user = ref<User>();

  function setUser(newUser: User) {
    user.value = newUser;
    localStorage.setItem(USER_INFO_KEY, JSON.stringify(newUser));
  }

  function clearUser() {
    user.value = void 0;
    localStorage.setItem(USER_INFO_KEY, '');
  }

  function getUser() {
    if (user.value)
      return user.value;
    const userStr = localStorage.getItem(USER_INFO_KEY);
    if (!userStr)
      return void 0;
    return user.value = JSON.parse(userStr) as User;
  }


  const token = ref('');

  function setToken(newToken: string) {
    token.value = newToken;
    localStorage.setItem(TOKEN_KEY, newToken);
  }

  function clearToken() {
    token.value = '';
    localStorage.setItem(TOKEN_KEY, '');
  }

  function getToken() {
    if (token.value)
      return token.value;
    const localToken = localStorage.getItem(TOKEN_KEY);
    return token.value = localToken ?? '';
  }

  return {
    setUser,
    clearUser,
    getUser,
    setToken,
    clearToken,
    getToken,
    user,
    token
  }
})
