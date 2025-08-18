import { parseError } from "src/api/error";
import CONSOLE from "src/utils/console";
import {
    AcceptInvite,
    Configuration,
    CreateClubRequest,
    DefaultApi,
    GetClubAdminsAdminRequest,
    GetClubMembersAdminRequest,
    RequiredError,
    ResponseError,
} from "src/api/generated";

/** Hyphen separated uuid */
export type UUID = string;

const configuration = new Configuration({
    basePath: window.location.origin,
});

const api = new DefaultApi(configuration);

/* eslint-disable */
export const Api = {
    admin: {
        admins: {
            getAll: () => handleError(api.getAdmins()),
        },
        clubs: {
            getAll: () => handleError(api.getClubsAdmin()),
            get: (uuid: UUID) => handleError(api.getClubAdmin({ uuid })),
            clubMembers: (req: GetClubMembersAdminRequest) => handleError(api.getClubMembersAdmin(req)),
            clubAdmins: (req: GetClubAdminsAdminRequest) => handleError(api.getClubAdminsAdminRaw(req)),
            create: (createClub: CreateClubRequest) =>
                handleError(api.createClubAdmin({ CreateClubRequest: createClub })),
            delete: (uuid: UUID) => handleError(api.deleteClubAdmin({ uuid })),
        },
        invites: {
            create: (invite: AdminCreateInviteRequest) =>
                handleError(api.adminCreateInvite({ AdminCreateInviteRequest: invite })),
        },
    },
    auth: {
        login: (username: string, password: string) => api.signIn({ SignInRequest: { username, password } }),
        logout: () => handleError(api.signOut()),
    },
    invites: {
        get: (uuid: UUID) => handleError(api.getInviteCommon({ uuid })),
        accepted: (uuid: UUID, req: AcceptInvite) =>
            handleError(
                api.acceptInvite({
                    uuid,
                    AcceptInvite: req,
                }),
            ),
    },
    me: {
        get: () => api.getMe(),
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
