import { Configuration, DefaultApi, GetClubMembersRequest } from "src/api/generated/club-admin";
import { handleError, UUID } from "src/api/api";

const clubAdminApi = new DefaultApi(new Configuration({ basePath: window.location.origin }));

/* eslint-disable */

export const ClubAdminApi = {
    club: {
        get: (uuid: UUID) => handleError(clubAdminApi.getClub({ club_uuid: uuid })),
        getMembers: (req: GetClubMembersRequest) => handleError(clubAdminApi.getClubMembers(req)),
    },
};

/* eslint-enable */
