import React, { useEffect } from "react";
import { FullWebsite } from "src/api/generated";
import { createFileRoute } from "@tanstack/react-router";
import { Api } from "src/api/api";
import { toast } from "react-toastify";
import Ws from "src/api/ws";
import HeadingLayout from "src/components/base/heading-layout";
import { Button } from "src/components/base/button";
import { Input } from "src/components/base/input";
import ChevronLeftIcon from "@heroicons/react/20/solid/ChevronLeftIcon";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "src/components/base/table";
import { Text } from "src/components/base/text";
import { XMarkIcon } from "@heroicons/react/20/solid";

/**
 * The properties for {@link WebsiteConfiguration}
 */
export type WebsiteConfigurationProps = {};

/**
 * The configuration for a website
 */
export default function WebsiteConfiguration(props: WebsiteConfigurationProps) {
    const { websiteId } = Route.useParams();

    const [website, setWebsite] = React.useState<FullWebsite>();

    const domainRef = React.useRef<HTMLInputElement>(null);
    const [newDomain, setNewDomain] = React.useState("");
    const [newDomainError, setNewDomainError] = React.useState(false);

    const [openDNSSettings, setOpenDNSSettings] = React.useState(false);

    const refresh = () => {
        Api.user.websites.get(websiteId).then((res) =>
            res.match(
                (website) => setWebsite(website),
                (err) => toast.error(err.message),
            ),
        );
    };

    const addDomain = () => {
        if (!validateDomain(newDomain)) {
            setNewDomainError(true);
            toast.error("Domain appears to be invalid");
            domainRef.current && domainRef.current.focus();
            return;
        }
        setNewDomainError(false);
        if (website === undefined) {
            return;
        }
        Api.user.websites.addDomain(website.uuid, newDomain).then((res) =>
            res.match(
                () => {
                    refresh();
                    setNewDomain("");
                },
                (err) => toast.error(err.message),
            ),
        );
    };

    const deploy = () => {
        website &&
            Api.user.websites.deploy(website.uuid).then((res) =>
                res.match(
                    (uuid) => {
                        const deployingId = toast.loading("Deploying");

                        const listener = Ws.addEventListener("message.DeployUpdate", (event) => {
                            if (event.task === uuid.uuid) {
                                if (event.state.res === "Success") {
                                    toast.update(deployingId, {
                                        render: "Deployment successful",
                                        autoClose: 3500,
                                        type: "success",
                                        isLoading: false,
                                    });
                                } else if (event.state.res === "Fail") {
                                    toast.update(deployingId, {
                                        render: "Deployment failed",
                                        autoClose: 3500,
                                        type: "error",
                                        isLoading: false,
                                    });
                                }

                                Ws.removeEventListener(listener);
                            }
                        });
                    },
                    (err) => toast.error(err.message),
                ),
            );
    };

    useEffect(() => {
        refresh();
    }, []);

    return (
        website && (
            <div className={"flex flex-col justify-start gap-6"}>
                <Button plain={true} className={"w-fit"} href={"/u/websites"}>
                    <ChevronLeftIcon />
                    <Text>Websites</Text>
                </Button>
                <HeadingLayout heading={`Edit Website ${website.name}`}>
                    <div className={"flex flex-col gap-3"}>
                        <Table dense={true} className={"text-zinc-800 dark:text-zinc-200"}>
                            <TableHead>
                                <TableRow>
                                    <TableHeader>Domain</TableHeader>
                                    <TableHeader className={"text-right"}>
                                        <span className={"sr-only"}>Action</span>
                                    </TableHeader>
                                </TableRow>
                            </TableHead>
                            <TableBody>
                                {website.domains.map((x) => (
                                    <TableRow key={x.uuid}>
                                        <TableCell>{x.domain}</TableCell>
                                        <TableCell className={"text-right"}>
                                            <Button
                                                plain={true}
                                                onClick={() => {
                                                    Api.user.websites.removeDomain(websiteId, x.uuid).then((res) =>
                                                        res.match(
                                                            () => refresh(),
                                                            (err) => toast.error(err.message),
                                                        ),
                                                    );
                                                }}
                                            >
                                                <XMarkIcon />
                                            </Button>
                                        </TableCell>
                                    </TableRow>
                                ))}
                            </TableBody>
                        </Table>
                        <form
                            method={"post"}
                            onSubmit={(ev) => {
                                ev.preventDefault();
                                addDomain();
                            }}
                            className={"flex gap-3"}
                        >
                            <Input
                                placeholder={"New domain"}
                                value={newDomain}
                                onChange={(ev) => {
                                    setNewDomain(ev.currentTarget.value);
                                }}
                                invalid={newDomainError}
                                ref={domainRef}
                            />
                            <Button type={"submit"} plain={true}>
                                Add
                            </Button>
                        </form>
                        <Button plain={true} onClick={() => setOpenDNSSettings(true)}>
                            Check DNS
                        </Button>
                    </div>
                    <div className={"flex justify-end gap-6"}>
                        <Button plain={true}>Delete website</Button>
                        <Button color={"orange"} onClick={deploy}>
                            Deploy configuration
                        </Button>
                    </div>
                </HeadingLayout>
            </div>
        )
    );
}

const re = new RegExp(`^(?=.{1,253}\\.?$)(?:(?!-|[^.]+_)[A-Za-z0-9-_]{1,63}(?<!-)(?:\\.|$)){2,}$`);

/**
 * Tests whether an input string seems to be a valid domain
 *
 * @param domain The string to test
 *
 * @returns boolean whether the test passed
 */
function validateDomain(domain: string) {
    return re.test(domain);
}

export const Route = createFileRoute("/_user/u/websites/$websiteId")({
    component: WebsiteConfiguration,
});
