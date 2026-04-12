module.exports = {
  platform: "github",
  endpoint: "https://api.github.com/",
  token: process.env.RENOVATE_TOKEN,
  gitAuthor: "Renovate Bot <renovate@omikron.dev>",
  repositories: ["myOmikron/bnv-manager-v2"],
  executionTimeout: 15,
  repositoryCache: "enabled",
  persistRepoData: true,
};
