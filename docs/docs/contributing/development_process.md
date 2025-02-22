# Development Process

## 🌳 Branching Strategy

We use a trunk-based development model where all changes are integrated into the `main` branch. Instead of maintaining multiple long-lived branches, we use tags and releases to manage different versions of the software.

## 🏷️ Versioning Strategy

We follow a modified semantic versioning scheme that includes development, beta, and release versions. Here's how our versioning works:

!!! bug inline end "🚑 On Hotfixes"
    When a critical bug needs to be fixed in a released version (say, 1.2.0) while main already contains work toward the next minor version (1.3.0-alpha.N), we use a temporary branch from the release tag. We create a branch from the v1.2.0 tag, make the necessary fix, and tag it as 1.2.1-beta.1 for testing. After verification, we release it as 1.2.1 and merge the hotfix back into main. This approach lets us make urgent fixes without interfering with ongoing development work, while still maintaining our versioning scheme and eventually getting all changes back into main. While this introduces a temporary branch, it's very short-lived (usually hours or days) and only used for critical fixes that need to avoid picking up in-progress work from main.

### 🛠️ Development Versions
- Format: `X.Y.Z-alpha.N+{git_hash}`
- Example: `1.2.3-alpha.1+a1b2c3d`
- These are automatically generated for each commit to `main`
- The git hash helps track exactly which commit produced the build

### 🧪 Beta Versions
- Format: `X.Y.Z-beta.N`
- Example: `1.2.3-beta.1`
- Beta numbers match their corresponding dev versions
- For example, `1.2.3-alpha.3+a1b2c3d` would become `1.2.3-beta.3`

### 🎯 Release Candidate Versions (Major Releases Only)
- Format: `X.Y.Z-rc.N`
- Example: `2.0.0-rc.1`
- Used only before major version changes
- Released after beta phase for additional testing

### ✨ Release Versions
- Format: `X.Y.Z`
- Example: `1.2.3`
- Released after successful beta/RC phase
- Drops all prerelease identifiers
- Next development version bumps the patch: `1.2.4-alpha.1+{git_hash}`

!!! example "📝 Version Flow Example"

    ```
    1.2.3-alpha.1+a1b2c3d  # Initial development version
    1.2.3-alpha.2+e4f5g6h  # More development
    1.2.3-alpha.3+i7j8k9l  # Ready for first beta
    1.2.3-beta.3         # First beta release
    1.2.3-alpha.4+m0n1o2p  # Fix based on beta feedback
    1.2.3-alpha.5+q3r4s5t  # More fixes
    1.2.3-alpha.6+u6v7w8x  # Ready for second beta
    1.2.3-beta.6         # Second beta release
    1.2.3                # Final release
    1.3.0-alpha.1+y9z0a1b  # Start next minor version
    ```

## 👷 Development Workflow

1. Always branch from latest `main`
2. Make your changes in small, focused commits
3. Write or update tests as needed
4. Run the test suite locally
5. Push your changes to your fork
6. Open a Pull Request against `main`
7. Once approved, your changes will be merged to `main`
8. CI will automatically create a dev version with your changes

## 🚀 Release Process

1. 💻 Development Phase
   - All work happens on `main`
   - Each merge triggers a dev release (`X.Y.Z-alpha.N+{git_hash}`)
   - Changes are tested in development builds

2. 🧪 Beta Phase
   - When ready for wider testing, a beta is tagged
   - Beta version matches latest dev number
   - Example: `1.2.3-alpha.5+abc123` → `1.2.3-beta.5`

3. 🎯 Release Candidate Phase (Major Versions Only)
   - After successful beta phase
   - Used for additional testing of major changes
   - Example: `2.0.0-beta.5` → `2.0.0-rc.1`

4. ✨ Release Phase
   - After successful testing
   - Drops prerelease identifiers
   - Example: `1.2.3-beta.5` → `1.2.3`
   - Next dev version bumps major, minor, or patch number depending on the changes planned for the next release
   - Example patch: `1.2.3` → `1.2.4-alpha.1+{git_hash}`
   - Example minor: `1.2.3` → `1.3.0-alpha.1+{git_hash}`
   - Example major: `1.2.3` → `2.0.0-alpha.1+{git_hash}`

## 📦 Package Publishing

Only the following versions are published to package registries:
- Beta releases (`X.Y.Z-beta.N`)
- Release candidates (`X.Y.Z-rc.N`)
- Final releases (`X.Y.Z`)

Development versions (`X.Y.Z-alpha.N+{git_hash}`) are built but not published.

!!! info "📈 Major, Minor, and Patch Releases"
    Patch releases (1.2.3 → 1.2.4) should be reserved for bug fixes, performance improvements, and security updates that don't change the public API. While we typically run these through a quick beta cycle for testing, they should be relatively straightforward changes that can be tested and released quickly. This is also how we handle hotfixes – through an accelerated beta cycle that might last just a day or two for urgent issues.

    Minor version bumps (1.2.0 → 1.3.0) are our main planning unit and represent meaningful feature additions. Each minor version should have a clear set of planned features or improvements, and work on these happens through development versions (1.3.0-alpha.N) until the features are ready for beta testing. This gives us a natural way to group related changes, plan our roadmap, and communicate upcoming features to users. It also means we can continue to release patches to the current minor version (1.2.4, 1.2.5) while working on the next one (1.3.0).

## 🎯 Testing and Stability

While we use a trunk-based approach with a single `main` branch, we maintain stability through our pre-release process:

### 🛠️ Development Builds
- Every commit to `main` produces a dev build
- These builds undergo automated testing
- Developers can use these builds for early testing
- Unstable but fast feedback loop

### 🧪 Beta Testing
- Beta releases mark code that's ready for wider testing
- Used to catch issues before final release
- Allows external users to test new features
- Important for catching real-world usage issues
- Multiple beta releases (beta.1, beta.2, etc.) may be needed

### 🎯 Release Candidates
- Extra stability gate for major versions
- Full regression testing
- Used to ensure backward compatibility
- Final chance for breaking issue discovery

This staged approach lets us maintain the simplicity of trunk-based development while ensuring proper testing and stability. Each stage (dev → beta → rc → release) represents increasing levels of stability and testing confidence.
