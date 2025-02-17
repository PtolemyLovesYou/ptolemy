# Identity & Access Management

Ptolemy provides a comprehensive access management system that helps teams collaborate effectively while maintaining strong security controls. Our role-based approach makes it easy to manage who can access what, whether they're querying data through our interface or connecting through API integrations. Let's dive into how it all works.

# Workspaces
Ptolemy organizes observability data into workspaces, providing a flexible way to manage access and permissions. Think of workspaces as separate environments where you can group related data and team members. Users can belong to multiple workspaces, and each workspace can have as many users as needed.
# Roles and Permissions
## Workspace Roles
Each workspace member is assigned one of these roles:

### User

- View and query workspace data
- Access workspace details and service API keys
- Perfect for team members who need to analyze data but don't need administrative access


### Manager

- Everything a User can do
- Create and delete service API keys
- Ideal for team leads who need to manage integrations and data access


### Admin

- Everything a Manager can do
- Add or remove workspace members
- Assign and modify user roles
- Delete the workspace
- Best for project owners and those responsible for workspace governance


## System-level Roles

### Admin

- Create and manage workspaces
- Add and remove users from Ptolemy
- Oversee system-wide access and permissions


### Sysadmin

- A special role designed with privacy in mind
- Limited to user management (create/delete)
- Intentionally restricted from accessing workspace data
- Helps sysadmins avoid unnecessary exposure to PII/PHI
- Only one sysadmin account per system, configured at the system level



# API Keys
Ptolemy offers two types of API keys to fit different authentication needs:

## User API Keys

- Tied to individual user accounts
- Provide read-only access to data in the user's workspaces
- Inherit all IAM permissions from the user
- Intended for integrations, management, and data exploration

## Service API Keys

- Owned by workspaces
- Can be created by workspace managers or admins
- Support flexible permission levels:
    - Read-only: For data consumption and analysis
    - Write-only: For data ingestion
    - Read-write: For full data access
- Intended for deployments, data integrations, and automations

Both types of API keys can be configured with optional expiration dates, which can be made mandatory at the system level. For detailed configuration options, check out our guide on [Configuration](../getting_started/configuration.md).

This role-based access control system ensures that your data remains secure while staying accessible to those who need it. Each permission level is carefully designed to balance security with usability, making it easy to manage access as your team and data needs grow.
