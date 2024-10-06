import {
    AuthApi,
    ChangeMeRequest,
    ClubsApi,
    Configuration,
    CreateClubRequest,
    CreateUserInviteRequestAdmin,
    CreateUserInviteRequestClubAdmin,
    RequiredError,
    ResponseError,
    UpdateClubRequest,
    UserInvitesApi,
    UsersApi,
    WebsitesApi,
} from "src/api/generated";
import { Err, Ok, Result } from "src/utils/result";
import CONSOLE from "src/utils/console";
import { ApiError, parseError, StatusCode } from "src/api/error";

/** Database id i.e. and u32 */
export type ID = number;

/** Hyphen separated uuid */
export type UUID = string;

const configuration = new Configuration({
    basePath: window.location.origin,
});
const authApi = new AuthApi(configuration);
const clubsApi = new ClubsApi(configuration);
const userInvitesApi = new UserInvitesApi(configuration);
const usersApi = new UsersApi(configuration);
const websitesApi = new WebsitesApi(configuration);

/* eslint-disable */

export const Api = {
    common: {
        auth: {
            login: (username: string, password: string) =>
                handleError(
                    authApi.login({
                        LoginRequest: {
                            username,
                            password,
                        },
                    }),
                ),
            logout: () => handleError(authApi.logout()),
        },
        users: {
            getMe: () => handleError(usersApi.getMe()),
            updateMe: (req: ChangeMeRequest) => handleError(usersApi.updateMe({ ChangeMeRequest: req })),
            changePassword: (current_pw: string, new_pw: string) =>
                handleError(
                    usersApi.changePassword({
                        ChangePwRequest: {
                            current_pw,
                            new_pw,
                        },
                    }),
                ),
        },
        userInvites: {
            get: (uuid: UUID) => handleError(userInvitesApi.getUserInvite({ uuid })),
            acceptWithPw: (uuid: UUID, password: string) =>
                handleError(
                    userInvitesApi.acceptInvitePw({
                        uuid,
                        AcceptInvitePwRequest: { password },
                    }),
                ),
        },
    },
    admin: {
        clubs: {
            all: () => handleError(clubsApi.getAllClubs()),
            get: (uuid: UUID) => handleError(clubsApi.getClub({ uuid })),
            create: (createClubRequest: CreateClubRequest) =>
                handleError(clubsApi.createClub({ CreateClubRequest: createClubRequest })),
            delete: (uuid: UUID) => handleError(clubsApi.deleteClub({ uuid })),
            update: (uuid: UUID, updateClubRequest: UpdateClubRequest) =>
                handleError(
                    clubsApi.updateClub({
                        uuid,
                        UpdateClubRequest: updateClubRequest,
                    }),
                ),
        },

        invites: {
            create: (createUserInviteRequest: CreateUserInviteRequestAdmin) =>
                handleError(
                    userInvitesApi.createInviteAdmin({ CreateUserInviteRequestAdmin: createUserInviteRequest }),
                ),
        },
    },
    clubAdmin: {
        userInvites: {
            create: (createUserInvite: CreateUserInviteRequestClubAdmin) =>
                handleError(
                    userInvitesApi.createInviteClubAdmin({ CreateUserInviteRequestClubAdmin: createUserInvite }),
                ),
        },
        users: {
            all: () => handleError(usersApi.getClubUsersClubAdmin()),
        },
    },
    user: {
        websites: {
            create: (name: string) => handleError(websitesApi.createWebsite({ CreateWebsiteRequest: { name } })),
            get: (uuid: UUID) => handleError(websitesApi.getWebsite({ uuid })),
            getAll: () => handleError(websitesApi.getAllWebsites()),
            update: (uuid: UUID, name: string) =>
                handleError(
                    websitesApi.updateWebsite({
                        uuid,
                        UpdateWebsiteRequest: { name },
                    }),
                ),
            addDomain: (uuid: UUID, domain: string) =>
                handleError(
                    websitesApi.addDomainToWebsite({
                        uuid,
                        AddDomainToWebsiteRequest: { domain },
                    }),
                ),
            removeDomain: (websiteUuid: UUID, domainUuid: UUID) =>
                handleError(
                    websitesApi.removeDomainFromWebsite({
                        website_uuid: websiteUuid,
                        domain_uuid: domainUuid,
                    }),
                ),
            delete: (uuid: UUID) => handleError(websitesApi.deleteWebsite({ uuid })),
            deploy: (uuid: UUID) => handleError(websitesApi.deployWebsite({ uuid })),
            checkDns: (uuid: UUID) => handleError(websitesApi.checkDns({ uuid })),
        },
    },
};

/* eslint-enable */

/**
 * Wraps a promise returned by the generated SDK which handles its errors and returns a {@link Result}
 */
export async function handleError<T>(promise: Promise<T>): Promise<Result<T, ApiError>> {
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
