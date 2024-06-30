# BNV Web Conf Updater

This server provides a simple JSON HTTP API with three endpoints:
- `/setup` to create or update the deployment of a website
- `/teardown` to remove the deployment of a website
- `/refresh` to go through all certificates and refresh them if needed
