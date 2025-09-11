# Working on projects

cv supports managing C/C++ projects, which define their dependencies in a cvproject.toml file.

## Creating a new project

You can create a new project using the `cv init` command:

```console
mkdir my_project
cd my_project
cv init
```

This will create a project with default settings, including the following files:

```
my_project/
├── .gitignore
├── README.md
├── src/main.c
└── cvproject.toml
```
