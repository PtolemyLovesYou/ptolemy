# Issue, Pull Request & Commit Guidelines

## 🎫 Issues

Every code change should start with an issue. Issues help track work, discuss implementation details, and maintain a clear project history.

### 🔄 One PR Per Issue Policy

We generally follow a one-PR-per-issue policy, with some pragmatic exceptions:

#### ✨ Why One PR Per Issue?
- 🔍 Clear traceability between changes and their motivations
- 📦 Forces proper task breakdown and manageable PR sizes
- 👀 Makes code reviews more focused and effective
- ⏪ Enables clean feature rollbacks if needed
- 📈 Provides clear project progress tracking

#### 🎭 Allowed Exceptions
- 🔨 Trivial changes (like typo fixes) may not need an issue
- 🔗 Multiple small, tightly related issues might be addressed in one PR (with clear documentation)
- 🔄 If implementation reveals an issue should split into multiple PRs, or multiple issues should combine, pause and restructure

When deviating from one-PR-per-issue, document your reasoning in the PR description.

### 📝 Issue Template
```markdown
# Problem
<!-- What needs to be done? -->

# Proposed Solution
<!-- How do you plan to solve it? -->

# Additional Context
<!-- Any extra information that might help? -->

# Acceptance Criteria
<!-- What needs to be true for this to be complete? -->
```

## 💌 Conventional Commits

We use conventional commits to maintain clear and standardized commit messages. Each commit message should follow this format:

```
<type>(<scope>): <description>

[optional body]

[optional footer(s)]
```

### 🏷️ Types
- `feat`: ✨ New feature
- `fix`: 🐛 Bug fix
- `docs`: 📚 Documentation changes
- `style`: 💅 Code style changes (formatting, missing semi-colons, etc)
- `refactor`: ♻️ Code refactoring
- `perf`: ⚡️ Performance improvements
- `test`: 🧪 Adding missing tests
- `chore`: 🔧 Build process or auxiliary tool changes

### 📋 Examples
```
feat(api): add endpoint for model metrics
fix(worker): resolve memory leak in batch processing
docs(readme): update installation instructions
perf(client): optimize large dataset handling
```

## 🚀 Pull Request Template

When opening a pull request, please use the following template:

```markdown
# Description
<!-- What does this PR do? -->

# Related Issue
<!-- Link to the issue this PR addresses -->
Closes #[issue-number]

# Type of Change
<!-- delete options that are not relevant -->
- 🚀 New feature
- 🔧 Bug fix
- 📚 Documentation
- 🔨 Breaking change
- ⚡️ Performance improvement
- 🧪 Test updates

# Testing
<!-- How were these changes tested? -->

# Breaking Changes
<!-- Does this PR introduce breaking changes? If yes, describe the impact and migration steps -->

# Checklist
- [ ] My code follows conventional commit guidelines
- [ ] I have added tests that prove my fix/feature works
- [ ] New and existing tests pass locally
- [ ] I have updated relevant documentation
- [ ] I have added metrics/monitoring for new features (if applicable)
```

### 🌟 Additional PR Guidelines
- 🎯 Keep PRs focused and reasonably sized
- 🔗 Link to related issues or discussions
- 💬 Respond to review comments promptly
- 🔄 Update your PR with main when there are conflicts
- 📸 Add screenshots or code examples for UI or API changes
