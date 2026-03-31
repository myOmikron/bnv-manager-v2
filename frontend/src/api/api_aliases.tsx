import { UUID } from "src/api/api";

const BASE = window.location.origin;

/** Schema for a mail alias */
export interface MailAliasSchema {
    /** UUID of the alias */
    uuid: string;
    /** Local part (before @) */
    local_part: string;
    /** Domain name */
    domain: string;
    /** Full alias address */
    full_address: string;
    /** Display name of the owning account */
    account_display_name: string;
    /** Status of the alias */
    status: "Pending" | "Approved" | "Rejected";
    /** When the alias was created */
    created_at: string;
}

/** Domain option for alias creation */
export interface DomainOptionSchema {
    /** UUID of the domain */
    uuid: string;
    /** Domain name */
    domain: string;
}

/** Request to propose a new alias */
export interface ProposeAliasRequest {
    /** Local part of the alias */
    local_part: string;
    /** UUID of the domain */
    domain_uuid: string;
}

/** Errors when proposing an alias */
export interface ProposeAliasError {
    /** The alias is already taken */
    alias_already_taken: boolean;
    /** The domain doesn't belong to the member's club */
    domain_not_in_club: boolean;
    /** Invalid local part */
    invalid_local_part: boolean;
}

/**
 * Success variant of a form result
 *
 * @template T The success value type
 */
interface FormResultOk<T> {
    /** Discriminator */
    result: "Ok";
    /** The success value */
    value: T;
}

/**
 * Error variant of a form result
 *
 * @template E The error value type
 */
interface FormResultErr<E> {
    /** Discriminator */
    result: "Err";
    /** The error value */
    error: E;
}

/**
 * Form result type matching the backend
 *
 * @template T The success value type
 * @template E The error value type
 */
export type FormResult<T, E> = FormResultOk<T> | FormResultErr<E>;

/**
 * Helper to make API requests
 *
 * @param method HTTP method
 * @param path URL path
 * @param body optional request body
 * @returns parsed JSON response
 */
async function request<T>(method: string, path: string, body?: unknown): Promise<T> {
    const res = await fetch(`${BASE}${path}`, {
        method,
        headers: body ? { "Content-Type": "application/json" } : {},
        body: body ? JSON.stringify(body) : undefined,
    });
    if (!res.ok) {
        const text = await res.text();
        throw new Error(`API error ${res.status}: ${text}`);
    }
    if (res.status === 204 || res.headers.get("content-length") === "0") {
        return undefined as T;
    }
    return res.json();
}

/** API for club member alias operations */
export const MemberAliasApi = {
    /**
     * Get all aliases for the logged-in member
     *
     * @returns list of aliases
     */
    getAll: () => request<MailAliasSchema[]>("GET", "/api/v1/frontend/club-member/aliases"),
    /**
     * Get available domains for the member's club
     *
     * @returns list of domains
     */
    getDomains: () => request<DomainOptionSchema[]>("GET", "/api/v1/frontend/club-member/aliases/domains"),
    /**
     * Propose a new alias
     *
     * @param req the proposal request
     * @returns form result with alias or error
     */
    propose: (req: ProposeAliasRequest) =>
        request<FormResult<MailAliasSchema, ProposeAliasError>>("POST", "/api/v1/frontend/club-member/aliases", req),
    /**
     * Delete an alias
     *
     * @param alias_uuid UUID of the alias to delete
     * @returns void
     */
    delete: (alias_uuid: UUID) => request<void>("DELETE", `/api/v1/frontend/club-member/aliases/${alias_uuid}`),
};

/** API for club admin alias operations */
export const ClubAdminAliasApi = {
    /**
     * Get all aliases for a club
     *
     * @param club_uuid UUID of the club
     * @returns list of aliases
     */
    getAll: (club_uuid: UUID) =>
        request<MailAliasSchema[]>("GET", `/api/v1/frontend/club-admin/clubs/${club_uuid}/aliases`),
    /**
     * Get pending aliases for a club
     *
     * @param club_uuid UUID of the club
     * @returns list of pending aliases
     */
    getPending: (club_uuid: UUID) =>
        request<MailAliasSchema[]>("GET", `/api/v1/frontend/club-admin/clubs/${club_uuid}/aliases/pending`),
    /**
     * Approve an alias
     *
     * @param club_uuid UUID of the club
     * @param alias_uuid UUID of the alias
     * @returns the approved alias
     */
    approve: (club_uuid: UUID, alias_uuid: UUID) =>
        request<MailAliasSchema>(
            "POST",
            `/api/v1/frontend/club-admin/clubs/${club_uuid}/aliases/${alias_uuid}/approve`,
        ),
    /**
     * Reject an alias
     *
     * @param club_uuid UUID of the club
     * @param alias_uuid UUID of the alias
     * @returns void
     */
    reject: (club_uuid: UUID, alias_uuid: UUID) =>
        request<void>("POST", `/api/v1/frontend/club-admin/clubs/${club_uuid}/aliases/${alias_uuid}/reject`),
    /**
     * Delete an alias
     *
     * @param club_uuid UUID of the club
     * @param alias_uuid UUID of the alias
     * @returns void
     */
    delete: (club_uuid: UUID, alias_uuid: UUID) =>
        request<void>("DELETE", `/api/v1/frontend/club-admin/clubs/${club_uuid}/aliases/${alias_uuid}`),
};
