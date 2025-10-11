import {
    Configuration,
    CreateClubRequest,
    CreateInviteRequestAdmin,
    CreateOidcProvider,
    DefaultApi,
    GetClubAdminsRequest,
    GetClubMembersRequest,
} from "src/api/generated/admin";
import { UUID } from "src/api/api";
import { RequiredError, ResponseError } from "src/api/generated/admin";
import { parseError } from "src/api/error";
import CONSOLE from "src/utils/console";

const adminApi = new DefaultApi(new Configuration({ basePath: window.location.origin }));

export const AdminApi = {
    superadmins: {
        getAll: () => handleError(adminApi.getAllSuperadmins()),
    },
    clubAdmins: {
        delete: (uuid: UUID) => handleError(adminApi.deleteClubAdmin({ uuid })),
    },
    clubs: {
        getAll: () => handleError(adminApi.getClubs()),
        get: (uuid: UUID) => handleError(adminApi.getClub({ uuid })),
        clubMembers: (req: GetClubMembersRequest) => handleError(adminApi.getClubMembers(req)),
        clubAdmins: (req: GetClubAdminsRequest) => handleError(adminApi.getClubAdmins(req)),
        invitedClubMembers: (uuid: UUID) => handleError(adminApi.getClubMemberInvites({ uuid })),
        invitedClubAdmins: (uuid: UUID) => handleError(adminApi.getClubAdminInvites({ uuid })),
        create: (createClub: CreateClubRequest) => handleError(adminApi.createClub({ CreateClubRequest: createClub })),
        delete: (uuid: UUID) => handleError(adminApi.deleteClub({ uuid })),
        associatedDomains: (uuid: UUID) => handleError(adminApi.getClubDomains({ uuid })),
    },
    domains: {
        unassociated: () => handleError(adminApi.getUnassociatedDomains()),
    },
    invites: {
        create: (invite: CreateInviteRequestAdmin) =>
            handleError(adminApi.createInvite({ CreateInviteRequestAdmin: invite })),
        retract: (invite_uuid: UUID) => handleError(adminApi.retractInvite({ uuid: invite_uuid })),
    },
    oidcProvider: {
        create: (req: CreateOidcProvider) => handleError(adminApi.createOidcProvider({ CreateOidcProvider: req })),
        all: () => handleError(adminApi.getAllOidcProviders()),
    },
};

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
            await parseError(e.response);
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
