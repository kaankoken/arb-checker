
# Code of Conduct

This project uses conventional commits & semantic versioning. Please read the following guidelines before contributing to this project.

You could also read the [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) documentation for more information. See the [semantic versioning](https://semver.org/) documentation for more information.

1. Use the conventional commit format for all commit messages.
2. Each commit message should consist of a type, scope (optional), and subject. If needed, you can include a longer description in the body or footer.
3. Use the following types:

    ```md
    **feat**: for adding new features
    **fix**: for fixing bugs
    **refactor**: for refactoring code without adding or changing features
    **perf**: for improving performance
    **docs**: for adding or modifying documentation
    **style**: for changes to the code style or formatting
    **test**: for adding or modifying tests
    **build**: for changes to the build process or tools
    **ci**: for changes to the continuous integration process or tools
    **chore**: for changes to the development process or tools that do not affect the code or tests
    **revert**: for reverting changes to the codebase
    ```

4. Use lowercase letters for types and scope.
5. Use sentence case for the subject, with no trailing punctuation.
6. Use the scope to describe the part of the codebase that was modified, if applicable.
7. Use imperative mood for the subject, beginning with a verb. For example, use "add new feature" instead of "added new feature".
8. Use the body to provide additional details about the changes made or the reasoning behind them, if needed.
9. Use the footer to provide any relevant information, such as issue references or breaking changes.

With these guidelines, a sample commit message might look like this:

```bash
feat(search): add search suggestions feature

This commit adds a new feature that displays search suggestions as users type in the search box. 

The new feature is implemented using a third-party library that provides real-time search suggestions based on user input.

Closes #123
```

## What is the difference between `build` and `ci`?

The `ci` and `build` commit message tags have slightly different meanings and purposes.

The `ci` tag is used to indicate changes that affect the continuous integration (CI) process, such as changes to build scripts, configuration files, or testing infrastructure. This tag is typically used for changes that do not affect the functionality of the code itself, but rather the process of building, testing, and deploying the code. For example, if you update the version of a testing library used in your project, you could use the `ci` tag to indicate that this change is related to the testing infrastructure.

On the other hand, the `build` tag is used to indicate changes that affect the compiled or built output of the code, such as changes to the build configuration or compiler flags. This tag is typically used for changes that affect the way the code is compiled, packaged, or deployed. For example, if you change the optimization level used by the Rust compiler, you could use the `build` tag to indicate that this change is related to the compiled output of the code.

In summary, the `ci` tag is used for changes that affect the CI process, while the `build` tag is used for changes that affect the compiled output of the code. While there is some overlap between these two tags, they are generally used to distinguish between different types of changes and make it easier to track and understand the evolution of the codebase over time.
