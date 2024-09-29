import { createFileRoute } from "@tanstack/react-router";

import React, { useEffect } from "react";
import BackButton from "src/components/base/back-button";
import { useTranslation } from "react-i18next";
import { Text } from "src/components/base/text";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import { FullClub } from "src/api/generated";
import Stats from "src/components/base/stats";
import { Heading } from "src/components/base/heading";

/**
 * The properties for {@link ClubView}
 */
export type ClubViewProps = {};

/**
 * The overview of a single club
 */
export default function ClubView(props: ClubViewProps) {
    const [t] = useTranslation();
    const [tC] = useTranslation("club-view");

    const { clubId } = Route.useParams();

    const [club, setClub] = React.useState<FullClub>();

    /**
     * Retrieve a club
     */
    const getClub = async () => {
        const res = await Api.admin.clubs.get(clubId);

        res.match(
            (c) => setClub(c),
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        getClub().then();
    }, [clubId]);

    if (!club) {
        return undefined;
    }

    return (
        <>
            <BackButton href={"/a/dashboard"}>
                <Text className={"!text-sm font-normal"}>{t("button.back")}</Text>
            </BackButton>
            <div className={"mt-6 flex w-full flex-col gap-6"}>
                <Heading>{tC("heading.club-overview", { club: club.name })}</Heading>
                <div className={"grid grid-cols-1 gap-6 sm:grid-cols-3"}>
                    <Stats key={"user-count"} label={tC("label.user-count")} value={club.user_count} />
                    <Stats key={"user-count"} label={tC("label.user-count")} value={club.user_count} />
                    <Stats key={"user-count"} label={tC("label.user-count")} value={club.user_count} />
                </div>
            </div>
        </>
    );
}

export const Route = createFileRoute("/_admin/a/clubs/$clubId")({
    component: ClubView,
});
