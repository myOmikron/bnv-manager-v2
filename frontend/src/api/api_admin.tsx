import {
    Configuration,
    CreateClubRequest,
    CreateInviteRequestAdmin,
    CreateOidcProvider,
    DefaultApi,
    GetClubAdminsRequest,
    GetClubMembersRequest,
} from "src/api/generated/admin";
import { handleError, UUID } from "src/api/api";

const adminApi = new DefaultApi(new Configuration({ basePath: window.location.origin }));

export const AdminApi = {
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
        create: (createClub: CreateClubRequest) => handleError(adminApi.createClub({ CreateClubRequest: createClub })),
        delete: (uuid: UUID) => handleError(adminApi.deleteClub({ uuid })),
        associatedDomains: (uuid: UUID) => handleError(adminApi.getClubDomains({ uuid })),
    },
    domains: {},
    invites: {
        create: (invite: CreateInviteRequestAdmin) =>
            handleError(adminApi.createInvite({ CreateInviteRequestAdmin: invite })),
    },
    oidcProvider: {
        create: (req: CreateOidcProvider) => handleError(adminApi.createOidcProvider({ CreateOidcProvider: req })),
        all: () => handleError(adminApi.getAllOidcProviders()),
    },
};
