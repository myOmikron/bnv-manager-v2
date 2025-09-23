import {
    Configuration,
    CreateMemberInviteRequest,
    DefaultApi,
    GetClubMembersRequest,
} from "src/api/generated/club-admin";
import { handleError, UUID } from "src/api/api";

const clubAdminApi = new DefaultApi(new Configuration({ basePath: window.location.origin }));

export const ClubAdminApi = {
    club: {
        get: (club_uuid: UUID) => handleError(clubAdminApi.getClub({ club_uuid })),
        getMembers: (req: GetClubMembersRequest) => handleError(clubAdminApi.getClubMembers(req)),
        getInvitedMembers: (club_uuid: UUID) => handleError(clubAdminApi.getClubMemberInvites({ club_uuid })),
    },
    invites: {
        create: (club_uuid: UUID, req: CreateMemberInviteRequest) =>
            handleError(clubAdminApi.createMemberInvite({ club_uuid, CreateMemberInviteRequest: req })),
    },
};
