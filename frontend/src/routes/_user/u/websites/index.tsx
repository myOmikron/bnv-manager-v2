import { createFileRoute } from "@tanstack/react-router";
import React, { useEffect } from "react";
import { SimpleWebsite } from "src/api/generated";
import HeadingLayout from "src/components/base/heading-layout";
import { Button } from "src/components/base/button";
import { PlusIcon } from "@heroicons/react/20/solid";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { useTranslation } from "react-i18next";
import { Api } from "src/api/api";
import { toast } from "react-toastify";

/**
 * The properties for {@link Websites}
 */
export type WebsitesProps = {};

/**
 * The configuration for the websites
 */
export default function Websites(props: WebsitesProps) {
    const [t] = useTranslation();
    const [tW] = useTranslation("website");

    const [websites, setWebsites] = React.useState<Array<SimpleWebsite>>([]);

    /**
     * Refresh the websites of the user
     */
    const refreshWebsites = async () => {
        const res = await Api.websites.getAll();

        res.match(
            (websites) => setWebsites(websites.websites),
            (err) => toast.error(err.message),
        );
    };

    useEffect(() => {
        refreshWebsites().then();
    }, []);

    return (
        <HeadingLayout
            heading={tW("heading.website-configuration")}
            headingChildren={
                <Button type={"button"} href={"/u/websites/create"}>
                    <PlusIcon />
                    {t("button.create")}
                </Button>
            }
        >
            <Table>
                <TableHead>
                    <TableRow>
                        <TableHeader>Name</TableHeader>
                        <TableHeader>Created at</TableHeader>
                        <TableHeader>Status</TableHeader>
                        <TableHeader>Last deployed</TableHeader>
                    </TableRow>
                </TableHead>
                <TableBody>
                    {websites.map((x) => (
                        <TableRow
                            key={x.uuid}
                            href={"/u/websites/$websiteId"}
                            params={{ websiteId: x.uuid }}
                            className={"dark:text-white"}
                        >
                            <TableCell>{x.name}</TableCell>
                            <TableCell>{new Date(x.created_at).toLocaleDateString()}</TableCell>
                            <TableCell></TableCell>
                            <TableCell>
                                {x.last_deployment ? new Date(x.last_deployment).toLocaleString() : ""}
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>
            </Table>
        </HeadingLayout>
    );
}

export const Route = createFileRoute("/_user/u/websites/")({
    component: Websites,
});
