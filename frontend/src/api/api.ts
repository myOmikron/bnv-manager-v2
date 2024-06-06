import {
    AuthApi,
    Configuration,
    LdapApi,
    RequiredError,
    ResponseError,
    UsersApi,
    WebsitesApi,
} from "./generated";
import { Err, Ok, Result } from "../utils/result";
import CONSOLE from "../utils/console";
import { ApiError, parseError, StatusCode } from "./error";

/** Database id i.e. and u32 */
export type ID = number;

/** Hyphen separated uuid */
export type UUID = string;

const configuration = new Configuration({
    basePath: window.location.origin,
});
const authApi = new AuthApi(configuration);
const usersApi = new UsersApi(configuration);
const ldapApi = new LdapApi(configuration);
const websitesApi = new WebsitesApi(configuration);

/* eslint-disable */

export const Api = {
    auth: {
        login: (username: string, password: string) =>
            handleError(
                authApi.login({
                    loginRequest: {
                        username,
                        password,
                    },
                }),
            ),
        logout: () => handleError(authApi.logout()),
    },
    ldap: {
        login: (username: string, password: string) =>
            handleError(
                ldapApi.loginLdap({
                    ldapLoginRequest: {
                        username,
                        password,
                    },
                }),
            ),
    },
    users: {
        getMe: () => handleError(usersApi.getMe()),
    },
    websites: {
        create: (name: string) =>
            handleError(
                websitesApi.createWebsite({ createWebsiteRequest: { name } }),
            ),
        get: (uuid: UUID) => handleError(websitesApi.getWebsite({ uuid })),
        getAll: () => handleError(websitesApi.getAllWebsites()),
        update: (uuid: UUID, name: string) =>
            handleError(
                websitesApi.updateWebsite({
                    uuid,
                    updateWebsiteRequest: { name },
                }),
            ),
        addDomain: (uuid: UUID, domain: string) =>
            handleError(
                websitesApi.addDomainToWebsite({
                    uuid,
                    addDomainToWebsiteRequest: { domain },
                }),
            ),
        removeDomain: (websiteUuid: UUID, domainUuid: UUID) =>
            handleError(
                websitesApi.removeDomainFromWebsite({
                    websiteUuid,
                    domainUuid,
                }),
            ),
        delete: (uuid: UUID) =>
            handleError(websitesApi.deleteWebsite({ uuid })),
    },
};

/* eslint-enable */

/**
 * Wraps a promise returned by the generated SDK which handles its errors and returns a {@link Result}
 */
export async function handleError<T>(
    promise: Promise<T>,
): Promise<Result<T, ApiError>> {
    try {
        return new Ok(await promise);
    } catch (e) {
        if (e instanceof ResponseError) {
            return new Err(await parseError(e.response));
        } else if (e instanceof RequiredError) {
            CONSOLE.error(e);
            return new Err({
                status_code: StatusCode.JsonDecodeError,
                message: "The server's response didn't match the spec",
            });
        } else {
            CONSOLE.error("Unknown error occurred:", e);
            return new Err({
                status_code: StatusCode.ArbitraryJSError,
                message: "Unknown error occurred",
            });
        }
    }
}
