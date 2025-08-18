import { createFileRoute } from "@tanstack/react-router";

import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import TablePagination from "src/components/table-pagination";
import { Text } from "src/components/base/text";

/**
 * Props for {@link ClubMembers}
 */
export type ClubMembersProps = {};

/**
 * Members of the club
 */
export default function ClubMembers(props: ClubMembersProps) {
    const [t] = useTranslation("admin-club-view");
    const [tg] = useTranslation();

    const params = Route.useParams();
    const data = Route.useLoaderData();
    const search = Route.useSearch();

    return (
        <>
            {data.total > 0 ? (
                <>
                    <Table>
                        <TableHead>
                            <TableRow>
                                <TableHeader>{t("label.username")}</TableHeader>
                                <TableHeader>{t("label.display-name")}</TableHeader>
                            </TableRow>
                        </TableHead>
                        <TableBody>
                            {data.items.map((item) => (
                                <TableRow key={item.uuid}>
                                    <TableCell>{item.username}</TableCell>
                                    <TableCell>{item.display_name}</TableCell>
                                </TableRow>
                            ))}
                        </TableBody>
                    </Table>
                    <TablePagination
                        href={"/a/clubs/$clubId/members"}
                        params={params}
                        maxPages={Math.ceil(data.total / LIMIT)}
                        currentPage={search.page}
                        getSearchParams={(newPage) => ({ page: newPage, search: search.search })}
                    />
                </>
            ) : (
                <Text>{t("label.no-members")}</Text>
            )}
        </>
    );
}

const LIMIT = 20;

/**
 * Parameter for this endpoint
 */
type SearchParams = {
    /** Current page that should be displayed */
    page: number;
    /** Search for a user */
    search?: string;
};

export const Route = createFileRoute("/_menu/a/clubs/$clubId/_club/members")({
    component: ClubMembers,
    // eslint-disable-next-line
    validateSearch: (search: Record<string, unknown>): SearchParams => {
        const page = Number(search?.page ?? 1);

        return {
            page: page <= 0 ? 1 : page,
            search: search?.search as string | undefined,
        };
    },
    // eslint-disable-next-line
    loaderDeps: ({ search: { page, search } }) => ({ page, search }),

    // eslint-disable-next-line
    loader: async ({ params, deps }) =>
        await Api.admin.clubs.clubMembers({
            uuid: params.clubId,
            limit: LIMIT,
            offset: (deps.page - 1) * LIMIT,
            search: deps.search,
        }),
});
