# Identity & Access Management

Ptolemy provides a comprehensive access management system that helps teams collaborate effectively while maintaining strong security controls. Our role-based approach makes it easy to manage who can access what, whether they're querying data through our interface or connecting through API integrations. Let's dive into how it all works.

# Workspaces
Ptolemy organizes observability data into workspaces, providing a flexible way to manage access and permissions. Think of workspaces as separate environments where you can group related data and team members. Users can belong to multiple workspaces, and each workspace can have as many users as needed.

!!! info "Decentralized Design"
    We've designed Ptolemy's workspace management with clear boundaries and distributed control in mind. Here's how it works:

    When creating a workspace, you must specify its initial admin - this can be yourself or another user. From there, workspace management is fully independent. Even system-level admins can create workspaces but can't delete them - only workspace admins have that power. This ensures each workspace maintains its autonomy and data governance.

    Most teams operate perfectly well with a single workspace. Multiple workspaces are primarily useful when you need strict isolation between different sets of observability data - for example, separating development and production environments, or maintaining boundaries between different business units.

    For some organizations, multiple Ptolemy deployments might make more sense than multiple workspaces. Consider this approach if you have distinct VPCs, different compliance requirements across teams, or need to optimize for latency across geographic regions.

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
