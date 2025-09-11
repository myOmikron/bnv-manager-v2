import { parseError } from "src/api/error";
import CONSOLE from "src/utils/console";
import {
    DefaultApi as CommonApi,
    Configuration as CommonConfiguration,
    AcceptInvite,
    ResponseError,
    RequiredError,
    UpdateMeRequest,
    SetPasswordRequest,
} from "src/api/generated/common";
import { ClubAdminApi } from "src/api/api_club_admins";
import { AdminApi } from "src/api/api_admin";
import { AuthApi } from "src/api/api_auth";

/** Hyphen separated uuid */
export type UUID = string;

const commonApi = new CommonApi(new CommonConfiguration({ basePath: window.location.origin }));

/* eslint-disable */
export const Api = {
    admin: AdminApi,
    auth: AuthApi,
    clubAdmins: ClubAdminApi,
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
            update: (req: UpdateMeRequest) => handleError(commonApi.updateMe({ UpdateMeRequest: req })),
            setPassword: (req: SetPasswordRequest) => handleError(commonApi.setPassword({ SetPasswordRequest: req })),
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
