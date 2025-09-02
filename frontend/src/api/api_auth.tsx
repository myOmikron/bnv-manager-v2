import { Configuration, DefaultApi } from "src/api/generated/auth";
import { handleError } from "src/api/api";

const authApi = new DefaultApi(new Configuration({ basePath: window.location.origin }));

/* eslint-disable */

export const AuthApi = {
    login: (username: string, password: string) => authApi.signIn({ SignInRequest: { username, password } }),
    logout: () => handleError(authApi.signOut()),
};

/* eslint-enable */
