module.exports = {
  platform: "github",
  endpoint: "https://api.github.com/",
  token: process.env.RENOVATE_TOKEN,
  gitAuthor: "Renovate Bot <renovate@omikron.dev>",
  gitPrivateKey: process.env.RENOVATE_GPG_KEY,
  repositories: ["myOmikron/bnv-manager-v2"],
  executionTimeout: 15,
  repositoryCache: "enabled",
  persistRepoData: true,
  logLevel: process.env.LOG_LEVEL || "info",
};
