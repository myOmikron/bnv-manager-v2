import { parseError } from "src/api/error";
import CONSOLE from "src/utils/console";
import {
    Configuration as AdminConfiguration,
    CreateClubRequest,
    CreateInviteRequest,
    CreateOidcProvider,
    CreateOidcProviderRequest,
    DefaultApi as AdminDefaultApi,
    GetClubAdminsRequest,
    GetClubMembersRequest,
    RequiredError,
    ResponseError,
} from "src/api/generated/admin";
import { DefaultApi as CommonApi, Configuration as CommonConfiguration, AcceptInvite } from "src/api/generated/common";
import { DefaultApi as AuthApi, Configuration as AuthConfiguration } from "src/api/generated/auth";

/** Hyphen separated uuid */
export type UUID = string;

const adminApi = new AdminDefaultApi(
    new AdminConfiguration({
        basePath: window.location.origin,
    }),
);
const commonApi = new CommonApi(new CommonConfiguration({ basePath: window.location.origin }));
const authApi = new AuthApi(new AuthConfiguration({ basePath: window.location.origin }));

/* eslint-disable */
export const Api = {
    admin: {
        superadmins: {
            getAll: () => handleError(adminApi.getAllSuperadmins()),
        },
        clubs: {
            getAll: () => handleError(adminApi.getClubs()),
            get: (uuid: UUID) => handleError(adminApi.getClub({ uuid })),
            clubMembers: (req: GetClubMembersRequest) => handleError(adminApi.getClubMembers(req)),
            clubAdmins: (req: GetClubAdminsRequest) => handleError(adminApi.getClubAdmins(req)),
            invitedClubMembers: (uuid: UUID) => handleError(adminApi.getClubMemberInvites({ uuid })),
            invitedClubAdmins: (uuid: UUID) => handleError(adminApi.getClubAdminInvites({ uuid })),
            create: (createClub: CreateClubRequest) =>
                handleError(adminApi.createClub({ CreateClubRequest: createClub })),
            delete: (uuid: UUID) => handleError(adminApi.deleteClub({ uuid })),
            associatedDomains: (uuid: UUID) => handleError(adminApi.getClubDomains({ uuid })),
        },
        domains: {
            unassociated: () => handleError(adminApi.getUnassociatedDomains()),
        },
        invites: {
            create: (invite: CreateInviteRequest) =>
                handleError(adminApi.createInvite({ CreateInviteRequest: invite })),
        },
        oidcProvider: {
            create: (req: CreateOidcProvider) => handleError(adminApi.createOidcProvider({ CreateOidcProvider: req })),
            all: () => handleError(adminApi.getAllOidcProviders()),
        },
    },
    auth: {
        login: (username: string, password: string) => authApi.signIn({ SignInRequest: { username, password } }),
        logout: () => handleError(authApi.signOut()),
    },
    common: {
        invites: {
            get: (uuid: UUID) => handleError(commonApi.getInviteCommon({ uuid })),
            accept: (uuid: UUID, req: AcceptInvite) =>
                handleError(
                    commonApi.acceptInvite({
                        uuid,
                        AcceptInvite: req,
                    }),
                ),
        },
        me: {
            get: () => commonApi.getMe(),
        },
    },
};

/* eslint-enable */

/**
 * Wraps a promise returned by the generated SDK which handles its errors and returns a {@link Result}
 *
 * @param promise The promise to wrap. This should be a promise defined in the generated part of the API
 *
 * @returns a new promise with a result that wraps errors from the API
 */
export async function handleError<T>(promise: Promise<T>): Promise<T> {
    try {
        return await promise;
    } catch (e) {
        let msg;
        if (e instanceof ResponseError) {
            const err = await parseError(e.response);
            msg = err.message;
        } else if (e instanceof RequiredError) {
            CONSOLE.error(e);
            msg = "The server's response didn't match the spec";
        } else {
            CONSOLE.error("Unknown error occurred:", e);
            msg = "Unknown error occurred";
        }
        throw msg;
    }
}
